cat << 'PATCHEOF' > delete_patch.patch
--- src/UserInterface/PythonTextTail.cpp
+++ src/UserInterface/PythonTextTail.cpp
@@ -798,16 +798,24 @@
 
 	if (m_CharacterTextTailMap.end() != itorCharacter)
 	{
-		DeleteTextTail(itorCharacter->second);
+		TTextTail* pTextTail = itorCharacter->second;
+		auto it = std::find(m_CharacterTextTailList.begin(), m_CharacterTextTailList.end(), pTextTail);
+		if (it != m_CharacterTextTailList.end())
+			m_CharacterTextTailList.erase(it);
+		DeleteTextTail(pTextTail);
 		m_CharacterTextTailMap.erase(itorCharacter);
 	}
 	else
 	{
 		Tracenf("CPythonTextTail::DeleteCharacterTextTail - Find VID[%d] Error", VirtualID);
 	}
 
 	if (m_ChatTailMap.end() != itorChat)
 	{
-		DeleteTextTail(itorChat->second);
+		TTextTail* pTextTail = itorChat->second;
+		DeleteTextTail(pTextTail);
 		m_ChatTailMap.erase(itorChat);
 	}
 }
 
@@ -822,8 +830,12 @@
 		Tracef(" CPythonTextTail::DeleteItemTextTail - None Item Text Tail\n");
 		return;
 	}
 
-	DeleteTextTail(itor->second);
+	TTextTail* pTextTail = itor->second;
+	auto it = std::find(m_ItemTextTailList.begin(), m_ItemTextTailList.end(), pTextTail);
+	if (it != m_ItemTextTailList.end())
+		m_ItemTextTailList.erase(it);
+	DeleteTextTail(pTextTail);
 	m_ItemTextTailMap.erase(itor);
 }
PATCHEOF
patch -p0 < delete_patch.patch
