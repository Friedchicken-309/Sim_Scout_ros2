#!/bin/bash
# ============================================================
# Scout 仿真一键启动脚本
#
# 用法:
#   ./run_sim.sh                      # 默认启动(Gazebo+RViz+手柄遥控)
#   ./run_sim.sh use_rviz:=false      # 不开 RViz
#   ./run_sim.sh use_joy:=false       # 不开手柄遥控
#   ./run_sim.sh gui:=false           # 无界面模式(后台跑物理仿真)
#   ./run_sim.sh x:=2.0 y:=1.0        # 指定出生点
#   ./run_sim.sh --build              # 强制重新构建后再启动
#
# 其余参数会原样透传给 ros2 launch
# ============================================================
set -e

# 工作区根目录 = 本脚本所在目录(保证在任意位置执行都正确)
WS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# ---------- 环境 ----------
if [ ! -f /opt/ros/humble/setup.bash ]; then
    echo "错误: 未找到 /opt/ros/humble/setup.bash, 请确认已安装 ROS 2 Humble"
    exit 1
fi
source /opt/ros/humble/setup.bash

# ---------- 构建 ----------
FORCE_BUILD=0
LAUNCH_ARGS=()
for arg in "$@"; do
    if [ "$arg" = "--build" ]; then
        FORCE_BUILD=1
    else
        LAUNCH_ARGS+=("$arg")
    fi
done

# 首次运行(无 install)或指定 --build 时, 只构建仿真需要的包
if [ ! -f "$WS_DIR/install/scout_description/share/scout_description/package.xml" ] \
   || [ "$FORCE_BUILD" = "1" ]; then
    echo "==> 构建 scout_description ..."
    cd "$WS_DIR"
    colcon build --packages-select scout_description
fi

source "$WS_DIR/install/setup.bash"

# ---------- 启动 ----------
echo "==> 启动 Scout 仿真 (参数: ${LAUNCH_ARGS[*]:-无})"
exec ros2 launch scout_description scout_gazebo.launch.py "${LAUNCH_ARGS[@]}"
