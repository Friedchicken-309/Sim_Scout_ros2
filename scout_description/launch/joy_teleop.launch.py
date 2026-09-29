import os

import launch
import launch_ros

from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node


def generate_launch_description():
    joy_config = os.path.join(get_package_share_directory('scout_description'),
                              'config', 'joy_teleop.yaml')

    joy_node = launch_ros.actions.Node(
        package='joy',
        executable='joy_node',
        name='joy_node',
        output='screen',
        parameters=[{
            'device_id': LaunchConfiguration('joy_device_id'),
            'autorepeat_rate': 20.0,
        }],
    )

    teleop_node = launch_ros.actions.Node(
        package='teleop_twist_joy',
        executable='teleop_node',
        name='teleop_twist_joy_node',
        output='screen',
        parameters=[joy_config],
    )

    return LaunchDescription([
        DeclareLaunchArgument('joy_device_id', default_value='0',
                              description='Joystick device id (/dev/input/js<X>)'),
        joy_node,
        teleop_node,
    ])
