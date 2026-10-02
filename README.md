# Scout 移动机器人仿真工程（ROS 2 Humble + Gazebo Classic）

本工程用于 Scout v2 四轮滑移转向（skid-steer）移动小车的仿真：默认加载室外小镇场景
（`small_city`），车头配有按 RealSense D435 参数仿真的深度相机，支持手柄/键盘遥控、
RViz 可视化与二次开发导航算法。

## 工程结构

| 包 | 说明 |
|---|---|
| `scout_description` | 小车 URDF/xacro 模型、仿真 world 与场景模型、launch、RViz 配置 |
| `scout_msgs` | Scout 相关消息定义 |
| `realsense_gazebo_plugin` | PAL Robotics 的 D435 Gazebo 仿真插件（源码 vendored，含本地补丁，见下） |
| `point_cloud_transport` | 上一插件依赖的消息中间件（源码 vendored，无需 apt 安装） |

> `realsense_gazebo_plugin` 相对上游（humble-devel, v3.2.0）做了两处修复：
> 1. 相机初始化推迟到第一个仿真 tick，修复无头 gzserver 启动即崩溃的问题；
> 2. 删除 `package.xml` 中 ROS1 遗留的 `gazebo_media_path` 导出，避免
>    `GAZEBO_RESOURCE_PATH` 指向无效目录导致渲染场景初始化失败。

## 环境要求与编译

- Ubuntu 22.04 / ROS 2 Humble / Gazebo Classic 11（`gazebo_ros`、`rviz2`、`joy`、`teleop_twist_joy` 等）
- 所有额外依赖已 vendored 到工作区，**无需 sudo 安装其它包**

```bash
cd scout_ros2
colcon build
source install/setup.bash
```

## 启动仿真

```bash
# 常规启动（Gazebo + RViz + 手柄遥控）
ros2 launch scout_description scout_gazebo.launch.py

# 纯无头（不启动 Gazebo 图形界面、RViz 和手柄）
ros2 launch scout_description scout_gazebo.launch.py gui:=false use_rviz:=false use_joy:=false

# 键盘遥控（替代手柄时）
ros2 run teleop_twist_keyboard teleop_twist_keyboard
```

## 常用 launch 参数

`scout_gazebo.launch.py` 支持以下参数（均有默认值，按需覆盖）：

| 参数 | 默认值 | 说明 |
|---|---|---|
| `world` | `worlds/small_city.world` | Gazebo world 文件路径（见下节） |
| `gui` | `true` | 是否启动 Gazebo 图形界面（gzclient） |
| `use_rviz` | `true` | 是否启动 RViz2 |
| `use_joy` | `true` | 是否启用手柄遥控（joy + teleop_twist_joy） |
| `joy_device_id` | `0` | 手柄设备号（`/dev/input/js<X>`） |
| `x` / `y` / `z` / `yaw` | `0` / `0` / `0.25` / `0` | 小车出生位置与朝向 |

示例——在指定位置出生、只跑 Gazebo 不开 RViz：

```bash
ros2 launch scout_description scout_gazebo.launch.py x:=20 y:=-2 z:=0.25 use_rviz:=false
```

## 更换仿真世界（world）

**临时更换**（一次启动生效）：传 `world:=` 参数，值为 world 文件的绝对或相对路径：

```bash
# 换回内置空世界
ros2 launch scout_description scout_gazebo.launch.py \
    world:=$(ros2 pkg prefix scout_description)/share/scout_description/worlds/scout_empty.world
```

**永久更换默认 world**：修改 `scout_description/launch/scout_gazebo.launch.py` 中
`DeclareLaunchArgument('world', ...)` 的 `default_value`，重新 `colcon build` 即可。

**新增 world 文件**：

1. world 文件（`.world` / SDF）放入 `scout_description/worlds/`；
2. world 引用到的所有 Gazebo 模型（`model://` 闭包，注意模型内部还可能嵌套引用其它模型）
   放入 `scout_description/models/`——launch 已自动把该目录追加进 `GAZEBO_MODEL_PATH`，
   因此不需要联网下载在线模型、也不依赖 `~/.gazebo/models` 缓存；
3. 重新 `colcon build` 后用 `world:=` 指定新文件，或把它设为默认。

当前内置：`small_city.world`（室外小镇：道路、建筑、树木、车辆、加油站，模型 36 个）
和 `scout_empty.world`（离线空世界）。

## 传感器与话题

**底盘（`gazebo_ros_diff_drive` 四轮滑移转向）**

| 话题 | 说明 |
|---|---|
| `/cmd_vel` | 速度指令（geometry_msgs/Twist） |
| `/odom` | 轮式里程计（世界系真值积分，含 odom→base_link TF） |
| `/joint_states` | 四轮关节角速度 |

**D435 深度相机（`realsense_gazebo_plugin`）**

| 话题 | 说明 |
|---|---|
| `/camera/color/image_raw` | 彩色图（rgb8，独立 69° FOV，远裁剪 200 m） |
| `/camera/aligned_depth_to_color/image_raw` | 深度图（16UC1，0.3–10 m，与真机驱动同格式） |
| `/camera/depth/color/points` | 带颜色点云（超量程点为 NaN，frame 为 `camera_depth_optical_frame`） |
| `/camera/infra1/image_raw`、`/camera/infra2/image_raw` | 红外流（1 Hz，仿真中按需调高） |
| `/camera/*/camera_info` | 各流内参 |

主要 TF 坐标系：`odom → base_link → base_footprint / camera_link → camera_{color,depth,infra1,infra2}_optical_frame`。

注意：相机话题为传感器 QoS（`BEST_EFFORT`），自写订阅节点时需匹配 QoS，否则收不到数据。

## RViz 显示

`rviz/scout_sim.rviz` 已配置：机器人模型、TF、`/camera/depth/color/points` 点云
（默认开启，Color Transformer = RGB8）。深度图与彩色图两个 Image 显示项默认关闭，
在 Displays 面板勾选 `D435 Depth Image` / `D435 RGB Image` 开启。

## 已知特性与注意事项

- **滑移转向打滑**：原地旋转时实际偏航角速度约为指令的 40%–60% 且左右不对称，这是
  Gazebo Classic ODE 摩擦模型 + `fdir1` 随轮转动的固有伪影；里程计使用世界系真值，
  不受打滑影响。需要更准的转向响应可在插件中调 `wheel_separation_multiplier`。
