cat << 'PATCHEOF' > pick_fix.patch
--- src/UserInterface/PythonTextTail.cpp
+++ src/UserInterface/PythonTextTail.cpp
@@ -915,7 +915,7 @@
 {
 	for (TTextTailList::iterator itor = m_ItemTextTailList.begin(); itor != m_ItemTextTailList.end(); ++itor)
 	{
-		TTextTail * pTextTail = itor->second;
+		TTextTail * pTextTail = *itor;
 
 		if (ixMouse >= pTextTail->x + pTextTail->xStart && ixMouse <= pTextTail->x + pTextTail->xEnd &&
 			iyMouse >= pTextTail->y + pTextTail->yStart && iyMouse <= pTextTail->y + pTextTail->yEnd)
PATCHEOF
patch --binary -p0 < pick_fix.patch
