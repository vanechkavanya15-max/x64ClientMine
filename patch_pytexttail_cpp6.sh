sed -i 's/for (TTextTailMap::iterator itor = m_ItemTextTailMap.begin(); itor != m_ItemTextTailMap.end(); ++itor)/for (TTextTailList::iterator itor = m_ItemTextTailList.begin(); itor != m_ItemTextTailList.end(); ++itor)/' src/UserInterface/PythonTextTail.cpp
sed -i 's/TTextTail \* pTextTail = itor->second;/TTextTail \* pTextTail = \*itor;/' src/UserInterface/PythonTextTail.cpp
sed -i 's/SelectItemName(itor->first);/SelectItemName(pTextTail->dwVirtualID);/' src/UserInterface/PythonTextTail.cpp
sed -i 's/return (itor->first);/return (pTextTail->dwVirtualID);/' src/UserInterface/PythonTextTail.cpp
