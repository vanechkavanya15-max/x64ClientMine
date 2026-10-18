cat << 'PATCHEOF' > build_fix.patch
--- src/UserInterface/PythonTextTail.cpp
+++ src/UserInterface/PythonTextTail.cpp
@@ -107,7 +107,7 @@
 
 		for (itorMap = m_CharacterTextTailMap.begin(); itorMap != m_CharacterTextTailMap.end(); ++itorMap)
 		{
-			UpdateDistance(pixelPos, *itorMap);
+			UpdateDistance(pixelPos, itorMap->second);
 		}
 
 		for (itorMap = m_ItemTextTailMap.begin(); itorMap != m_ItemTextTailMap.end(); ++itorMap)
@@ -432,13 +432,13 @@
 	TTextTailMap::iterator itor;
 	for (itor = m_CharacterTextTailMap.begin(); itor != m_CharacterTextTailMap.end(); ++itor)
 	{
-		TTextTail * pTextTail = *itor;
+		TTextTail * pTextTail = itor->second;
 		if (pTextTail->fDistanceFromPlayer < 3500.0f)
 			ShowCharacterTextTail(itor->first);
 	}
 	for (itor = m_ItemTextTailMap.begin(); itor != m_ItemTextTailMap.end(); ++itor)
 	{
-		TTextTail * pTextTail = *itor;
+		TTextTail * pTextTail = itor->second;
 		if (pTextTail->fDistanceFromPlayer < 3500.0f)
 			ShowItemTextTail(itor->first);
 	}
@@ -451,7 +451,7 @@
 	if (m_CharacterTextTailMap.end() == itor)
 		return;
 
-	TTextTail * pTextTail = *itor;
+	TTextTail * pTextTail = itor->second;
 
 	if (m_CharacterTextTailList.end() != std::find(m_CharacterTextTailList.begin(), m_CharacterTextTailList.end(), pTextTail))
 	{
@@ -482,7 +482,7 @@
 	if (m_ItemTextTailMap.end() == itor)
 		return;
 
-	TTextTail * pTextTail = *itor;
+	TTextTail * pTextTail = itor->second;
 
 	if (m_ItemTextTailList.end() != std::find(m_ItemTextTailList.begin(), m_ItemTextTailList.end(), pTextTail))
 	{
@@ -748,7 +748,7 @@
 	if (m_CharacterTextTailMap.end() == itorCharacter)
 		return;
 
-	TTextTail * pTextTail = *itorCharacter;
+	TTextTail * pTextTail = itorCharacter->second;
 	pTextTail->pTextInstance->SetColor(c_rColor);
 	pTextTail->Color = c_rColor;
 }
@@ -759,7 +759,7 @@
 	if (m_ItemTextTailMap.end() == itor)
 		return;
 
-	TTextTail * pTextTail = *itor;
+	TTextTail * pTextTail = itor->second;
 
 	if (strlen(c_szName) > 0)
 	{
@@ -933,7 +933,7 @@
 	if (m_CharacterTextTailMap.end() == itor)
 		return;
 
-	TTextTail * pTextTail = *itor;
+	TTextTail * pTextTail = itor->second;
 	pTextTail->TitleColor = c_rColor;
 
 	CGraphicTextInstance *& prTitle = pTextTail->pTitleTextInstance;
@@ -957,7 +957,7 @@
 	if (m_CharacterTextTailMap.end() == itor)
 		return;
 
-	TTextTail * pTextTail = *itor;
+	TTextTail * pTextTail = itor->second;
 
 	if (pTextTail->pTitleTextInstance)
 	{
@@ -978,7 +978,7 @@
 	if (m_CharacterTextTailMap.end() == itor)
 		return;
 
-	TTextTail * pTextTail = *itor;
+	TTextTail * pTextTail = itor->second;
 	pTextTail->LevelColor = c_rColor;
 
 	CGraphicTextInstance *& prLevel = pTextTail->pLevelTextInstance;
@@ -1004,7 +1004,7 @@
 	if (m_CharacterTextTailMap.end() == itor)
 		return;
 
-	TTextTail * pTextTail = *itor;
+	TTextTail * pTextTail = itor->second;
 
 	if (pTextTail->pLevelTextInstance)
 	{
@@ -1020,7 +1020,7 @@
 
 	for (TTextTailMap::iterator itor = m_CharacterTextTailMap.begin(); itor != m_CharacterTextTailMap.end(); ++itor)
 	{
-		TTextTail* pTextTail = *itor;
+		TTextTail* pTextTail = itor->second;
 
 		CInstanceBase* pInstance = CPythonCharacterManager::Instance().GetInstancePtr(itor->first);
 		if (!pInstance)
PATCHEOF
patch -p0 < build_fix.patch
