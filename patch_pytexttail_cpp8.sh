sed -i 's/TTextTail \* pTextTail = \*itor;/TTextTail \* pTextTail = itor->second;/g' src/UserInterface/PythonTextTail.cpp
sed -i 's/TTextTail\* pTextTail = \*itor;/TTextTail\* pTextTail = itor->second;/g' src/UserInterface/PythonTextTail.cpp
sed -i 's/UpdateDistance(pixelPos, \*itorMap);/UpdateDistance(pixelPos, itorMap->second);/g' src/UserInterface/PythonTextTail.cpp
