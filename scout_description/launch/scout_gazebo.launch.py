import launch
import launch_ros

from launch import LaunchDescription
from launch.actions import (DeclareLaunchArgument, IncludeLaunchDescription,
                            SetEnvironmentVariable)
from launch.conditions import IfCondition
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch_ros.actions import Node
from launch_ros.parameter_descriptions import ParameterValue
from launch_ros.substitutions import FindPackageShare
from launch.substitutions import (EnvironmentVariable, FindExecutable,
                                  LaunchConfiguration, Command,
                                  PathJoinSubstitution)


def generate_launch_description():
    model_name = 'scout_v2.gazebo.xacro'

    robot_description_content = Command([
        PathJoinSubstitution([FindExecutable(name='xacro')]), ' ',
        PathJoinSubstitution(
            [FindPackageShare('scout_description'), 'urdf', model_name]
        ),
        ' use_gazebo:=true',
    ])

    # outdoor world (small_city) needs the models vendored in this package
    gazebo_model_path = [EnvironmentVariable('GAZEBO_MODEL_PATH',
                                             default_value=''), ':',
                         PathJoinSubstitution([FindPackageShare('scout_description'),
                                               'models'])]

    gazebo = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(PathJoinSubstitution(
            [FindPackageShare('gazebo_ros'), 'launch', 'gazebo.launch.py'])),
        launch_arguments={
            'verbose': 'true',
            'world': LaunchConfiguration('world'),
            'gui': LaunchConfiguration('gui'),
        }.items(),
    )

    robot_state_publisher = launch_ros.actions.Node(
        package='robot_state_publisher',
        executable='robot_state_publisher',
        name='robot_state_publisher',
        output='screen',
        parameters=[{
            'use_sim_time': True,
            'robot_description': ParameterValue(robot_description_content,
                                                value_type=str),
        }],
    )

    # base_link rides 0.2348 m above the ground (wheel offset + radius),
    # spawn slightly higher and let it settle
    spawn_entity = launch_ros.actions.Node(
        package='gazebo_ros',
        executable='spawn_entity.py',
        output='screen',
        arguments=[
            '-topic', 'robot_description',
            '-entity', 'scout_v2',
            '-x', LaunchConfiguration('x'),
            '-y', LaunchConfiguration('y'),
            '-z', LaunchConfiguration('z'),
            '-Y', LaunchConfiguration('yaw'),
        ],
    )

    rviz2 = launch_ros.actions.Node(
        package='rviz2',
        executable='rviz2',
        name='rviz2',
        output='screen',
        condition=IfCondition(LaunchConfiguration('use_rviz')),
        arguments=['-d', PathJoinSubstitution(
            [FindPackageShare('scout_description'), 'rviz', 'scout_sim.rviz'])],
        parameters=[{'use_sim_time': True}],
    )

    joy_teleop = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(PathJoinSubstitution(
            [FindPackageShare('scout_description'), 'launch',
             'joy_teleop.launch.py'])),
        launch_arguments={
            'joy_device_id': LaunchConfiguration('joy_device_id'),
        }.items(),
        condition=IfCondition(LaunchConfiguration('use_joy')),
    )

    return launch.LaunchDescription([
        DeclareLaunchArgument('use_rviz', default_value='true',
                              description='Start RViz2 alongside Gazebo'),
        DeclareLaunchArgument('use_joy', default_value='true',
                              description='Start joystick teleop (joy + teleop_twist_joy)'),
        DeclareLaunchArgument('joy_device_id', default_value='0',
                              description='Joystick device id (/dev/input/js<X>)'),
        DeclareLaunchArgument('gui', default_value='false',
                              description='Start the Gazebo GUI (gzclient)'),
        DeclareLaunchArgument('world',
                              default_value=PathJoinSubstitution(
                                  [FindPackageShare('scout_description'),
                                   'worlds', 'small_city.world']),
                              description='Gazebo world file to load'),
        DeclareLaunchArgument('x', default_value='0.0',
                              description='Spawn x position'),
        DeclareLaunchArgument('y', default_value='0.0',
                              description='Spawn y position'),
        DeclareLaunchArgument('z', default_value='0.25',
                              description='Spawn z position'),
        DeclareLaunchArgument('yaw', default_value='0.0',
                              description='Spawn yaw orientation'),
        SetEnvironmentVariable('GAZEBO_MODEL_PATH', gazebo_model_path),
        robot_state_publisher,
        gazebo,
        spawn_entity,
        rviz2,
        joy_teleop,
    ])
