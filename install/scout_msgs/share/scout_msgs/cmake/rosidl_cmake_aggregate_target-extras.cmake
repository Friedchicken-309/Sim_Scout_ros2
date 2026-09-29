# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target scout_msgs::scout_msgs
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${scout_msgs_TARGETS}.
if(scout_msgs_TARGETS AND NOT TARGET scout_msgs::scout_msgs)
  add_library(scout_msgs::scout_msgs INTERFACE IMPORTED)
  set_target_properties(scout_msgs::scout_msgs PROPERTIES
    INTERFACE_LINK_LIBRARIES "${scout_msgs_TARGETS}")
endif()
