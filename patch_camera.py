import re

with open('src/EterLib/Camera.cpp', 'r') as f:
    content = f.read()

replacement = """	// Zachowaj Up wektor jako czysty wektor gora (0, 0, 1) bez odwracania
	SetUp(D3DXVECTOR3(0.0f, 0.0f, 1.0f));
}"""

content = re.sub(r'	// Zachowaj Up wektor jako czysty wektor gora \(0, 0, 1\) bez odwracania\n\s*SetUp\(D3DXVECTOR3\(0\.0f, 0\.0f, 1\.0f\)\);\n\n\s*m_fRoll \+= fRollDegree;\n\n\s*if \(m_fRoll > 360\.0f\)\n\s*m_fRoll -= 360\.0f;\n\s*else if \(m_fRoll < -360\.0f\)\n\s*m_fRoll \+= 360\.0f;\n\}', replacement, content)

with open('src/EterLib/Camera.cpp', 'w') as f:
    f.write(content)
