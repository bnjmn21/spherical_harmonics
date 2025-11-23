# Applies some mesh optimization modifiers to all selected objects

import bpy

for o in bpy.context.selected_objects:
    o.modifiers.new("Decimate", "DECIMATE")
    o.modifiers["Decimate"].decimate_type = 'DISSOLVE'
    o.modifiers["Decimate"].angle_limit = 0.0436332
    o.modifiers.new("Decimate.001", "DECIMATE")
    o.modifiers["Decimate.001"].ratio = 0.25
    o.modifiers.new("Triangulate", "TRIANGULATE")
