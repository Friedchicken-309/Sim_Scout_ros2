# Scout 仿真工程交接文档（Agent 开发用）

> 面向下一个项目：基于第一视角 RGB + 深度图 + 里程计开发 Agent，
> 后期增加激光雷达做简单语义建图。
> 日常使用说明见 `README.md`，本文只讲"对接开发"需要知道的事。

## 0. 一页速览

```bash
# 终端 1：启动仿真（无头 + 不开 RViz，适合跑 Agent）
cd ~/Documents/zjl/Agentic_Robotics/scout_ros2 && source install/setup.bash
ros2 launch scout_description scout_gazebo.launch.py gui:=false use_rviz:=false use_joy:=false

# 终端 2：你的 Agent 工作区，source 本工程作为 underlay 后再跑自己的节点
source ~/Documents/zjl/Agentic_Robotics/scout_ros2/install/setup.bash
# ... 启动你的 Agent
```

Agent 消费的四个接口：

| 接口 | 话题/坐标系 | 一句话说明 |
|---|---|---|
| 第一视角 RGB | `/camera/color/image_raw` | rgb8，848×480，69° FOV |
| 深度图 | `/camera/aligned_depth_to_color/image_raw` | **16UC1，单位毫米**，0 = 无效 |
| 里程计 | `/odom`（+ TF `odom→base_link`） | 世界系真值位姿，无漂移 |
| 运动控制 | `/cmd_vel` | Twist；注意打滑（见 §3） |

## 1. 工程现状

| 包 | 内容 | 你需要关心的程度 |
|---|---|---|
| `scout_description` | 车体模型、world、传感器定义、launch | 加激光雷达时改这里 |
| `realsense_gazebo_plugin` | D435 仿真插件（PAL v3.2.0 + 2 处本地补丁） | 一般不动 |
| `point_cloud_transport` | 插件依赖，vendored | 不动 |
| `scout_base` / `scout_msgs` | 实车 CAN 驱动 | 仿真用不到 |

关键文件索引：

| 文件 | 作用 |
|---|---|
| `scout_description/launch/scout_gazebo.launch.py` | 总入口；world/GUI/RViz/手柄/出生点参数 |
| `scout_description/urdf/scout_v2.gazebo.xacro` | 仿真专用：差速插件、轮摩擦、**D435 四传感器定义** |
| `scout_description/urdf/scout_v2.xacro` | 车体几何/质量（实车描述共用） |
| `scout_description/worlds/small_city.world` | 默认室外小镇场景 |
| `scout_description/models/` | 场景模型（36 个，自包含离线） |
| `scout_description/rviz/scout_sim.rviz` | RViz 配置（含点云显示） |

## 2. 传感器接口细节（重点）

### 2.1 RGB `/camera/color/image_raw`

- `rgb8`，848×480，30 Hz 目标（无头软渲染实际约 5–7 Hz；带 GUI/GPU 明显更高）
- QoS：**BEST_EFFORT**（传感器数据），你的订阅必须匹配：

```python
from rclpy.qos import qos_profile_sensor_data
sub = node.create_subscription(Image, '/camera/color/image_raw', cb, qos_profile_sensor_data)
```

- `header.frame_id = camera_color_optical_frame`（光学系：z 前、x 右、y 下）

### 2.2 深度图 `/camera/aligned_depth_to_color/image_raw`

- `16UC1`，与真机 RealSense 驱动同格式：**`depth_m = raw / 1000.0`**
- 有效量程 0.3–10 m；**超量程像素 = 0**（不是 NaN，做投影时按无效处理）
- **与 RGB 是像素级精确对齐的**：两个虚拟相机共光心（零基线）且 FOV/分辨率/内参
  完全一致（实测两路 camera_info 的 K 矩阵相同：848×480, fx=fy=616.97, cx=424, cy=240），
  同一下标像素严格对应同一方向光线，可直接做 RGB-D 逐像素融合，无视差无需配准
  （这相当于真机 D435 `aligned_depth_to_color` 的定义——深度已投影进彩色帧坐标系）。
  注意内参话题的位置：**深度流内参在
  `/camera/aligned_depth_to_color/camera_info`**（跟随图像话题命名；
  `depth/camera_info` 这个参数名在 ROS 2 版插件里是遗留未用的）。
- 嫌 16UC1 麻烦可直接订阅 `/camera/depth/color/points`（PointCloud2，
  自带颜色、NaN 剔除无效点，frame 为 `camera_depth_optical_frame`），
  或以点云为几何真值、以 RGB 做语义输入。

### 2.3 里程计 `/odom`

- 由 `gazebo_ros_diff_drive` 发布，`odometry_source=1`：**取自 Gazebo 世界系真值位姿**
  （不是轮子积分），因此无漂移、无打滑误差——开发期很好用，但**上实车后行为会变**，
  别把"零漂移"写死进 Agent 假设。
- QoS 为默认 RELIABLE，普通订阅即可；同时发布 TF `odom → base_link`。
- 仿真时间：Gazebo 发布 `/clock`。Agent 节点建议 `use_sim_time:=true`，
  保证与相机/TF 时间戳一致。

### 2.4 坐标系（TF 树）

```
odom → base_link → base_footprint
                 → camera_link → camera_color_optical_frame
                               → camera_depth_optical_frame
                               → camera_infra1/2_optical_frame
```

- `base_link` 原点 = 车体几何中心，位于轮距中心上方 0.2348 m
- `camera_link` 在车头（base_link 系 x=+0.44, z=+0.10）
- 所有 `*_optical_frame` = 光学约定（z 前、x 右、y 下），图像/点云数据都在光学系

## 3. 运动控制 `/cmd_vel`

- `geometry_msgs/Twist`；`linear.x` 前进 m/s，`angular.z` 角速度 rad/s
- **滑移转向打滑（实测）**：原地旋转实际角速度只有指令的 40%–60%，且左右不对称
  （+1.0 → ~0.57，−1.0 → ~0.36 rad/s）；行驶中转向也有类似衰减。
  Agent 做闭环/轨迹跟踪时不要假设指令=实际，以 `/odom` 反馈为准。
  想改善可调 `scout_v2.gazebo.xacro` 里插件的 `wheel_separation_multiplier`（建议 1.3–1.6）。
- 插件限幅：`max_wheel_torque=50`，`max_wheel_acceleration=1.0`
- 出生点可用 launch 参数 `x y z yaw` 调整（默认 (0,0,0.25,0)，在主路中间）

## 4. 新项目（Agent 工作区）的组织建议

- **推荐**：Agent 独立工作区，把本工程 install 作为 underlay：

```bash
mkdir -p ~/agent_ws/src && cd ~/agent_ws
source ~/Documents/zjl/Agentic_Robotics/scout_ros2/install/setup.bash
colcon build   # 你的包能 find_package / 订阅所有仿真话题
```

- 仿真与 Agent 分终端运行；需要重复批量实验时可把 launch 包进 ROS 2 service/脚本。
- **无头运行**：`gui:=false` 只是不开 Gazebo 窗口，相机渲染仍需要 X 显示
  （桌面终端天然满足）；纯命令行/容器/CI 环境用 `xvfb-run -a ros2 launch ...`。
- 跑批前检查残留进程：`pgrep -af gzserver`，有就 `pkill -f gzserver`
  （残留 gzserver 会占用 Gazebo master 端口，导致下一次启动行为诡异）。

## 5. 后续加激光雷达（做语义建图时）

改 `scout_description/urdf/scout_v2.gazebo.xacro`，追加（放 `</robot>` 前）：

```xml
<link name="lidar_link"/>
<joint name="lidar_joint" type="fixed">
    <parent link="base_link"/>
    <child link="lidar_link"/>
    <origin xyz="0 0 0.16" rpy="0 0 0"/>   <!-- 车顶，按安装位置调 -->
</joint>
<gazebo reference="lidar_link">
    <sensor type="ray" name="lidar">
        <update_rate>10</update_rate>
        <visualize>false</visualize>
        <ray>
            <scan><horizontal>
                <samples>720</samples>
                <min_angle>-2.35619</min_angle>  <!-- ±135° 共 270° -->
                <max_angle>2.35619</max_angle>
            </horizontal></scan>
            <range><min>0.12</min><max>30.0</max></range>
            <noise><type>gaussian</type><stddev>0.01</stddev></noise>
        </ray>
        <plugin name="lidar_plugin" filename="libgazebo_ros_ray_sensor.so">
            <ros><remapping>~/out:=scan</remapping></ros>
            <frame_name>lidar_link</frame_name>
        </plugin>
    </sensor>
</gazebo>
```

要点：`frame_name` 必须与 link 同名；`min` 盲区要大于机身；纯命令行环境 `ray` 比
`gpu_ray` 稳；雷达点在 `lidar_link` 系（x 前），与相机的光学系不同，注意变换。
改完 `colcon build --packages-select scout_description` 即生效。

## 6. 已知坑速查

| 症状 | 原因/处理 |
|---|---|
| 订阅相机话题收不到数据，无报错 | QoS 不匹配，相机流是 BEST_EFFORT（§2.1） |
| 深度值都是 0 或特别大 | 0=超量程；记得 `/1000.0` 转米（§2.2） |
| RGB-D 投影错位 | 已不可复现：两流共光心、内参一致（§2.2）；若仍错位检查是否混用了红外流的 camera_info |
| 转弯半径比指令大很多 | 滑移打滑，闭环用 /odom 反馈（§3） |
| 相机话题全都没有 | 之前残留 gzserver 占端口，或无 X 显示（渲染被禁用，日志见 "Rendering is disabled"） |
| RViz 深度图全黑 | 显示层问题；用点云核对数据，或自转 8-bit 查看 |
| 仿真时间与墙钟不一致 | Agent 节点设 `use_sim_time:=true` |

## 7. 快速自检命令

```bash
ros2 topic hz /camera/color/image_raw        # 相机流活着（匹配 QoS）
ros2 topic hz /odom                          # 里程计活着
ros2 topic echo /camera/depth/camera_info --once   # 深度流内参（投影用）
ros2 topic echo /clock --once                # 仿真时间
ros2 run tf2_ros tf2_echo odom base_link     # 里程计 TF
gz stats                                     # 实时率（软渲染低是正常）
```

## 8. 许可证提示

`small_city.world` 及部分场景模型为 GPLv3（来源：开源 world 收集仓库）；
`realsense_gazebo_plugin` 为 Apache 2.0。仿真自用无碍，成果若闭源发布需注意剥离。
