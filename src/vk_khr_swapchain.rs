use crate::*;

/*

<extension name="VK_KHR_swapchain" number="2" type="device" depends="VK_KHR_surface" author="KHR" contact="James Jones @cubanismo,Ian Elliott @ianelliottus" supported="vulkan,vulkansc">
<require>
<enum value="70"                                                name="VK_KHR_SWAPCHAIN_SPEC_VERSION"/>
<enum value="&quot;VK_KHR_swapchain&quot;"                      name="VK_KHR_SWAPCHAIN_EXTENSION_NAME"/>
<enum offset="0" extends="VkStructureType"                      name="VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR"/>
<enum offset="1" extends="VkStructureType"                      name="VK_STRUCTURE_TYPE_PRESENT_INFO_KHR"/>
<enum offset="2" extends="VkImageLayout"                        name="VK_IMAGE_LAYOUT_PRESENT_SRC_KHR"/>
<enum offset="3" extends="VkResult"                             name="VK_SUBOPTIMAL_KHR"/>
<enum offset="4" extends="VkResult" dir="-"                     name="VK_ERROR_OUT_OF_DATE_KHR"/>
<enum offset="0" extends="VkObjectType"                         name="VK_OBJECT_TYPE_SWAPCHAIN_KHR"/>
<type name="VkSwapchainCreateFlagBitsKHR"/>
<type name="VkSwapchainCreateFlagsKHR"/>
<type name="VkSwapchainCreateInfoKHR"/>
<type name="VkSwapchainKHR"/>
<type name="VkPresentInfoKHR"/>
<command name="vkCreateSwapchainKHR"/>
<command name="vkDestroySwapchainKHR"/>
<command name="vkGetSwapchainImagesKHR"/>
<command name="vkAcquireNextImageKHR"/>
<command name="vkQueuePresentKHR"/>
</require>
<require depends="VK_VERSION_1_1">
<comment>This duplicates definitions in VK_KHR_device_group below</comment>
<enum extends="VkStructureType" extnumber="61"  offset="7"      name="VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR"/>
<enum extends="VkStructureType" extnumber="61"  offset="8"      name="VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR"/>
<enum extends="VkStructureType" extnumber="61"  offset="9"      name="VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR"/>
<enum extends="VkStructureType" extnumber="61"  offset="10"     name="VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR"/>
<enum extends="VkStructureType" extnumber="61"  offset="11"     name="VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR"/>
<enum extends="VkStructureType" extnumber="61"  offset="12"     name="VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR"/>
<enum bitpos="0" extends="VkSwapchainCreateFlagBitsKHR"         name="VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR" comment="Allow images with VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT"/>
<type name="VkImageSwapchainCreateInfoKHR"/>
<type name="VkBindImageMemorySwapchainInfoKHR"/>
<type name="VkAcquireNextImageInfoKHR"/>
<type name="VkDeviceGroupPresentModeFlagBitsKHR"/>
<type name="VkDeviceGroupPresentModeFlagsKHR"/>
<type name="VkDeviceGroupPresentCapabilitiesKHR"/>
<type name="VkDeviceGroupPresentInfoKHR"/>
<type name="VkDeviceGroupSwapchainCreateInfoKHR"/>
<command name="vkGetDeviceGroupPresentCapabilitiesKHR"/>
<command name="vkGetDeviceGroupSurfacePresentModesKHR"/>
<command name="vkGetPhysicalDevicePresentRectanglesKHR"/>
<command name="vkAcquireNextImage2KHR"/>
<enum bitpos="1" extends="VkSwapchainCreateFlagBitsKHR"         name="VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR"     comment="Swapchain is protected"/>
</require>
</extension>

*/
