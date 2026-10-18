import sys

with open("src/GameLib/MapOutdoorWater.cpp", "r") as f:
    content = f.read()

content = content.replace("CalculateWaterAnimationFrameIndex(ELTimer_GetMSec())", "CalculateWaterAnimationFrameIndex(dwCurTime)")

with open("src/GameLib/MapOutdoorWater.cpp", "w") as f:
    f.write(content)
