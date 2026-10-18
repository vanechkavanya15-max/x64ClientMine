cat << 'PATCHEOF' > src/UserInterface/PythonTextTail.cpp.patch
--- src/UserInterface/PythonTextTail.cpp
+++ src/UserInterface/PythonTextTail.cpp
@@ -1086,11 +1086,19 @@
 
 void CPythonTextTail::Clear()
 {
+	for (auto& pair : m_CharacterTextTailMap)
+		DeleteTextTail(pair.second);
 	m_CharacterTextTailMap.clear();
+
+	for (auto& pair : m_ItemTextTailMap)
+		DeleteTextTail(pair.second);
+	m_ItemTextTailMap.clear();
+
+	for (auto& pair : m_ChatTailMap)
+		DeleteTextTail(pair.second);
+	m_ChatTailMap.clear();
+
 	m_CharacterTextTailList.clear();
-	m_ItemTextTailMap.clear();
 	m_ItemTextTailList.clear();
-	m_ChatTailMap.clear();
 
 	m_TextTailPool.Clear();
PATCHEOF
patch -p0 < src/UserInterface/PythonTextTail.cpp.patch
