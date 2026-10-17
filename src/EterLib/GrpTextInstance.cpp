#include "StdAfx.h"
#include "GrpTextInstance.h"
#include "StateManager.h"
#include "RenderOwnership.h"
#include "Render/DynamicQuadBatcher.h"
#include "IME.h"
#include "TextTag.h"
#include "EterBase/Utils.h"

#include <unordered_map>
#include <utf8.h>

// Forward declaration to avoid header conflicts
extern bool IsRTL();

const float c_fFontFeather = 0.5f;

CDynamicPool<CGraphicTextInstance> CGraphicTextInstance::ms_kPool;

static int gs_mx = 0;
static int gs_my = 0;

static std::wstring gs_hyperlinkText;

void CGraphicTextInstance::Hyperlink_UpdateMousePos(int x, int y)
{
	gs_mx = x;
	gs_my = y;
	gs_hyperlinkText = L"";
}

int CGraphicTextInstance::Hyperlink_GetText(char* buf, int len)
{
	if (gs_hyperlinkText.empty())
		return 0;

	int written = WideCharToMultiByte(CP_UTF8, WC_ERR_INVALID_CHARS, gs_hyperlinkText.c_str(), (int)gs_hyperlinkText.length(), buf, len, nullptr, nullptr);

	return (written > 0) ? written : 0;
}

int CGraphicTextInstance::__DrawCharacter(CGraphicFontTexture * pFontTexture, wchar_t text, DWORD dwColor, wchar_t prevChar)
{
	CGraphicFontTexture::TCharacterInfomation* pInsCharInfo = pFontTexture->GetCharacterInfomation(text);

	if (pInsCharInfo)
	{
		// Round kerning to nearest pixel to keep glyphs on the pixel grid.
		// Fractional offsets cause bilinear interpolation blur in D3D9.
		float kern = floorf(pFontTexture->GetKerning(prevChar, text) + 0.5f);

		m_dwColorInfoVector.push_back(dwColor);
		m_pCharInfoVector.push_back(pInsCharInfo);
		m_kernVector.push_back(kern);

		m_textWidth += (int)(pInsCharInfo->advance + kern);
		m_textHeight = std::max((WORD)pInsCharInfo->height, m_textHeight);
		return (int)(pInsCharInfo->advance + kern);
	}

	return 0;
}

void CGraphicTextInstance::__GetTextPos(DWORD index, float* x, float* y)
{
	index = std::min((size_t)index, m_pCharInfoVector.size());

	float sx = 0;
	float sy = 0;
	float fFontMaxHeight = 0;

	for(DWORD i=0; i<index; ++i)
	{
		if (sx+float(m_pCharInfoVector[i]->width) > m_fLimitWidth)
		{
			sx = 0;
			sy += fFontMaxHeight;
		}

		sx += float(m_pCharInfoVector[i]->advance);
		fFontMaxHeight = std::max(float(m_pCharInfoVector[i]->height), fFontMaxHeight);
	}

	*x = sx;
	*y = sy;
}

void CGraphicTextInstance::Update()
{
	if (m_isUpdate)
		return;

	// Get space height first for empty text cursor rendering
	int spaceHeight = 12; // default fallback
	if (!m_roText.IsNull() && !m_roText->IsEmpty())
	{
		CGraphicFontTexture* pFontTexture = m_roText->GetFontTexturePointer();
		if (pFontTexture)
		{
			CGraphicFontTexture::TCharacterInfomation* pSpaceInfo = pFontTexture->GetCharacterInfomation(L' ');
			spaceHeight = pSpaceInfo ? pSpaceInfo->height : 12;
		}
	}

	auto ResetState = [&, spaceHeight]()
		{
			m_pCharInfoVector.clear();
			m_kernVector.clear();
			m_dwColorInfoVector.clear();
			m_hyperlinkVector.clear();
			m_textWidth = 0;
			m_textHeight = spaceHeight; // Use space height instead of 0 for cursor rendering
			m_computedRTL = IsRTL(); // Use global RTL setting
			m_isUpdate = true;
		};

	if (m_roText.IsNull() || m_roText->IsEmpty())
	{
		ResetState();
		return;
	}

	CGraphicFontTexture* pFontTexture = m_roText->GetFontTexturePointer();
	if (!pFontTexture)
	{
		ResetState();
		return;
	}

	m_pCharInfoVector.clear();
	m_kernVector.clear();
	m_dwColorInfoVector.clear();
	m_hyperlinkVector.clear();

	m_textWidth = 0;
	m_textHeight = spaceHeight;

	const char* utf8 = m_stText.c_str();
	const int utf8Len = (int)m_stText.size();
	DWORD dwColor = m_dwTextColor;

	m_pCharInfoVector.reserve(utf8Len);
	m_kernVector.reserve(utf8Len);
	m_dwColorInfoVector.reserve(utf8Len);

	// UTF-8 -> UTF-16 conversion using instance buffer to avoid heap allocations
	m_wTextBuf.resize((size_t)utf8Len + 1u, 0);
	int wTextLen = MultiByteToWideChar(CP_UTF8, MB_ERR_INVALID_CHARS, utf8, utf8Len, m_wTextBuf.data(), (int)m_wTextBuf.size());

	// If strict UTF-8 conversion fails, try lenient mode (replaces invalid sequences)
	if (wTextLen <= 0)
	{
		// Try lenient conversion (no MB_ERR_INVALID_CHARS flag)
		wTextLen = MultiByteToWideChar(CP_UTF8, 0, utf8, utf8Len, m_wTextBuf.data(), (int)m_wTextBuf.size());

		if (wTextLen <= 0)
		{
			ResetState();
			return;
		}
	}

	// Set computed RTL based on global setting
	m_computedRTL = IsRTL();

	// Secret mode: draw '*' instead of actual characters
	if (m_isSecret)
	{
		wchar_t prevCh = 0;
		for (int i = 0; i < wTextLen; ++i)
		{
			__DrawCharacter(pFontTexture, L'*', dwColor, prevCh);
			prevCh = L'*';
		}

		pFontTexture->UpdateTexture();
		m_isUpdate = true;
		return;
	}

	// === RENDERING APPROACH ===
	// Use BuildVisualBidiText_Tagless() and BuildVisualChatMessage() from utf8.h
	// These functions handle Arabic shaping, BiDi reordering, and chat formatting properly

	// Special handling for chat messages
	if (m_isChatMessage && !m_chatName.empty() && !m_chatMessage.empty())
	{
		std::wstring wName = Utf8ToWide(m_chatName);
		std::wstring wMsg = Utf8ToWide(m_chatMessage);

		// Check if message has tags (hyperlinks)
		bool msgHasTags = (std::find(wMsg.begin(), wMsg.end(), L'|') != wMsg.end());

		if (!msgHasTags)
		{
			// No tags: Use BuildVisualChatMessage() for simple BiDi
			std::vector<wchar_t> visual = BuildVisualChatMessage(
				wName.data(), (int)wName.size(),
				wMsg.data(), (int)wMsg.size(),
				m_computedRTL);

			wchar_t prevCh = 0;
			for (size_t i = 0; i < visual.size(); ++i)
			{
				__DrawCharacter(pFontTexture, visual[i], dwColor, prevCh);
				prevCh = visual[i];
			}

			pFontTexture->UpdateTexture();
			m_isUpdate = true;
			return;
		}
		else
		{
			// Has tags (hyperlinks): Rebuild as "Message : Name" or "Name : Message"
			// then use tag-aware rendering below
			if (m_computedRTL)
			{
				// RTL: "Message : Name"
				m_stText = m_chatMessage + " : " + m_chatName;
			}
			else
			{
				// LTR: "Name : Message" (original format)
				m_stText = m_chatName + " : " + m_chatMessage;
			}

			// Re-convert to wide chars for tag-aware processing below
			const char* utf8 = m_stText.c_str();
			const int utf8Len = (int)m_stText.size();
			m_wTextBuf.resize((size_t)utf8Len + 1u, 0);
			wTextLen = MultiByteToWideChar(CP_UTF8, MB_ERR_INVALID_CHARS, utf8, utf8Len, m_wTextBuf.data(), (int)m_wTextBuf.size());

			if (wTextLen <= 0)
			{
				wTextLen = MultiByteToWideChar(CP_UTF8, 0, utf8, utf8Len, m_wTextBuf.data(), (int)m_wTextBuf.size());
				if (wTextLen <= 0)
				{
					ResetState();
					return;
				}
			}
			// Fall through to tag-aware rendering below
		}
	}

	// Check if text contains tags or RTL
	const bool hasTags = (std::find(m_wTextBuf.begin(), m_wTextBuf.begin() + wTextLen, L'|') != (m_wTextBuf.begin() + wTextLen));
	bool hasRTL = false;
	for (int i = 0; i < wTextLen; ++i)
	{
		if (IsRTLCodepoint(m_wTextBuf[i]))
		{
			hasRTL = true;
			break;
		}
	}

	// Tag-aware BiDi rendering: Parse tags, apply BiDi per segment, track colors/hyperlinks
	// OPTIMIZED: Use helper lambda to eliminate code duplication (was repeated 5+ times)
	if (hasRTL || hasTags)
	{
		DWORD currentColor = dwColor;
		int hyperlinkStep = 0; // 0=normal, 1=collecting metadata, 2=visible hyperlink
		std::wstring hyperlinkMetadata;

		m_currentSegment.clear();

		SHyperlink currentHyperlink;
		currentHyperlink.sx = currentHyperlink.ex = 0;

		// In chat RTL, force RTL base direction so prefixes like "[hyperlink]" don't flip the paragraph to LTR.
		const bool forceRTLForBidi = (m_isChatMessage && m_computedRTL);

		// OPTIMIZED: Single helper function for flushing segments (eliminates 5x code duplication)
		auto FlushSegment = [&](DWORD segColor) -> int
		{
			if (m_currentSegment.empty())
				return 0;

			int totalWidth = 0;

			// Apply BiDi transformation using optimized BuildVisualBidiText_Tagless
			std::vector<wchar_t> visual = BuildVisualBidiText_Tagless(
				m_currentSegment.data(), (int)m_currentSegment.size(), forceRTLForBidi);

			wchar_t prevCh = m_pCharInfoVector.empty() ? 0 : 0; // no prev across segments
			for (size_t j = 0; j < visual.size(); ++j)
			{
				int w = __DrawCharacter(pFontTexture, visual[j], segColor, prevCh);
				totalWidth += w;
				prevCh = visual[j];
			}

			m_currentSegment.clear();
			return totalWidth;
		};

		// Prepend glyphs to the already-built draw list (used to place hyperlink before message in RTL chat).
		auto PrependGlyphs = [&](CGraphicFontTexture* pFontTexture,
		                         const std::vector<wchar_t>& chars,
		                         DWORD color,
		                         int& outWidth)
		{
			outWidth = 0;

			m_newCharInfos.clear();
			m_newColors.clear();
			m_newKerns.clear();
			m_newCharInfos.reserve(chars.size());
			m_newColors.reserve(chars.size());
			m_newKerns.reserve(chars.size());

			for (size_t k = 0; k < chars.size(); ++k)
			{
				auto* pInfo = pFontTexture->GetCharacterInfomation(chars[k]);
				if (!pInfo)
					continue;

				m_newCharInfos.push_back(pInfo);
				m_newColors.push_back(color);
				m_newKerns.push_back(0.0f);

				outWidth += pInfo->advance;
				m_textHeight = std::max((WORD)pInfo->height, m_textHeight);
			}

			m_pCharInfoVector.insert(m_pCharInfoVector.begin(), m_newCharInfos.begin(), m_newCharInfos.end());
			m_dwColorInfoVector.insert(m_dwColorInfoVector.begin(), m_newColors.begin(), m_newColors.end());
			m_kernVector.insert(m_kernVector.begin(), m_newKerns.begin(), m_newKerns.end());

			for (auto& link : m_hyperlinkVector)
			{
				link.sx += outWidth;
				link.ex += outWidth;
			}

			m_textWidth += outWidth;
		};

		// Parse text with tags
		for (int i = 0; i < wTextLen;)
		{
			int tagLen = 0;
			std::wstring tagExtra;
			int tagType = GetTextTag(&m_wTextBuf[i], wTextLen - i, tagLen, tagExtra);

			if (tagType == TEXT_TAG_COLOR)
			{
				// Flush current segment before changing color
				currentHyperlink.ex += FlushSegment(currentColor);
				currentColor = htoi(tagExtra.c_str(), 8);
				i += tagLen;
			}
			else if (tagType == TEXT_TAG_RESTORE_COLOR)
			{
				// Flush segment before restoring color
				currentHyperlink.ex += FlushSegment(currentColor);
				currentColor = dwColor;
				i += tagLen;
			}
			else if (tagType == TEXT_TAG_HYPERLINK_START)
			{
				hyperlinkStep = 1;
				hyperlinkMetadata.clear();
				i += tagLen;
			}
			else if (tagType == TEXT_TAG_HYPERLINK_END)
			{
				if (hyperlinkStep == 1)
				{
					// End of metadata, start visible section
					// Flush any pending non-hyperlink segment first
					currentHyperlink.ex += FlushSegment(currentColor);

					hyperlinkStep = 2;
					currentHyperlink.text = hyperlinkMetadata;
					currentHyperlink.sx = currentHyperlink.ex; // Start hyperlink at current cursor position
				}
				else if (hyperlinkStep == 2)
				{
					// End of visible section - render hyperlink text with proper Arabic handling
					// In RTL chat: we want the hyperlink chunk to appear BEFORE the message, even if logically appended.
					if (!m_currentSegment.empty())
					{
						// Reusable instance buffer for visible rendering (avoids reallocations)
						m_visibleToRender.clear();

						// Find bracket positions: [ ... ]
						int openBracket = -1, closeBracket = -1;
						for (size_t idx = 0; idx < m_currentSegment.size(); ++idx)
						{
							if (m_currentSegment[idx] == L'[' && openBracket == -1)
								openBracket = (int)idx;
							else if (m_currentSegment[idx] == L']' && closeBracket == -1)
								closeBracket = (int)idx;
						}

						if (openBracket >= 0 && closeBracket > openBracket)
						{
							// Keep '['
							m_visibleToRender.push_back(L'[');

							// Extract inside content and apply BiDi
							m_content.assign(
								m_currentSegment.begin() + openBracket + 1,
								m_currentSegment.begin() + closeBracket);

							// FIX: Use false to let BiDi auto-detect direction from content
							// This ensures English items like [Sword+9] stay LTR
							// while Arabic items like [درع فولاذي+9] are properly RTL
							std::vector<wchar_t> visual = BuildVisualBidiText_Tagless(
								m_content.data(), (int)m_content.size(), false);

							m_visibleToRender.insert(m_visibleToRender.end(), visual.begin(), visual.end());

							// Keep ']'
							m_visibleToRender.push_back(L']');
						}
						else
						{
							// No brackets: apply BiDi to whole segment
							// FIX: Use false to let BiDi auto-detect direction from content
							std::vector<wchar_t> visual = BuildVisualBidiText_Tagless(
								m_currentSegment.data(), (int)m_currentSegment.size(), false);

							m_visibleToRender.insert(m_visibleToRender.end(), visual.begin(), visual.end());
						}

						// Ensure a space AFTER the hyperlink chunk (so it becomes "[hyperlink] اختبار...")
						m_visibleToRender.push_back(L' ');

						// Key behavior:
						// In RTL chat, place hyperlink BEFORE the message by prepending glyphs.
						if (m_isChatMessage && m_computedRTL)
						{
							int addedWidth = 0;
							PrependGlyphs(pFontTexture, m_visibleToRender, currentColor, addedWidth);

							// Record the hyperlink range at the beginning (0..addedWidth)
							currentHyperlink.sx = 0;
							currentHyperlink.ex = addedWidth;
							m_hyperlinkVector.push_back(currentHyperlink);
						}
						else
						{
							// LTR or non-chat: keep original "append" behavior
							currentHyperlink.sx = currentHyperlink.ex;
							wchar_t prevCh = 0;
							for (size_t j = 0; j < m_visibleToRender.size(); ++j)
							{
								int w = __DrawCharacter(pFontTexture, m_visibleToRender[j], currentColor, prevCh);
								currentHyperlink.ex += w;
								prevCh = m_visibleToRender[j];
							}
							m_hyperlinkVector.push_back(currentHyperlink);
						}
					}

					hyperlinkStep = 0;
					m_currentSegment.clear();
				}
				i += tagLen;
			}
			else // TEXT_TAG_PLAIN or TEXT_TAG_TAG
			{
				if (hyperlinkStep == 1)
				{
					// Collecting hyperlink metadata (hidden)
					hyperlinkMetadata.push_back(m_wTextBuf[i]);
				}
				else
				{
					// Add to current segment
					// Will be BiDi-processed for normal text, or rendered directly for hyperlinks
					m_currentSegment.push_back(m_wTextBuf[i]);
				}
				i += tagLen;
			}
		}

		// Flush any remaining segment using optimized helper
		currentHyperlink.ex += FlushSegment(currentColor);

		pFontTexture->UpdateTexture();
		m_isUpdate = true;
		return;
	}

	// Simple LTR rendering for plain text (no tags, no RTL)
	// Just draw characters in logical order
	{
		wchar_t prevCh = 0;
		for (int i = 0; i < wTextLen; ++i)
		{
			__DrawCharacter(pFontTexture, m_wTextBuf[i], dwColor, prevCh);
			prevCh = m_wTextBuf[i];
		}
	}

	pFontTexture->UpdateTexture();
	m_isUpdate = true;
}

CGraphicTextInstance::STextAlignment CGraphicTextInstance::ComputeAlignmentOffset() const
{
	STextAlignment align;

	if (m_computedRTL)
	{
		switch (m_hAlign)
		{
			case HORIZONTAL_ALIGN_LEFT:
				align.offsetX = -(float)m_textWidth;
				break;

			case HORIZONTAL_ALIGN_CENTER:
				align.offsetX = -float(m_textWidth / 2);
				break;

			case HORIZONTAL_ALIGN_RIGHT:
			default:
				align.offsetX = 0.0f;
				break;
		}
	}
	else
	{
		switch (m_hAlign)
		{
			case HORIZONTAL_ALIGN_RIGHT:
				align.offsetX = -(float)m_textWidth;
				break;

			case HORIZONTAL_ALIGN_CENTER:
				align.offsetX = -float(m_textWidth / 2);
				break;

			case HORIZONTAL_ALIGN_LEFT:
			default:
				align.offsetX = 0.0f;
				break;
		}
	}

	switch (m_vAlign)
	{
		case VERTICAL_ALIGN_BOTTOM:
			align.offsetY = -(float)m_textHeight;
			break;

		case VERTICAL_ALIGN_CENTER:
			align.offsetY = -float(m_textHeight) / 2.0f;
			break;

		case VERTICAL_ALIGN_TOP:
		default:
			align.offsetY = 0.0f;
			break;
	}

	return align;
}

const std::vector<STextQuad>& CGraphicTextInstance::LayoutGlyphs(RECT* pClipRect)
{
	m_layoutQuads.clear();

	if (!m_isUpdate)
	{
		Update();
	}

	CGraphicText* pkText = m_roText.GetPointer();
	CGraphicFontTexture* pFontTexture = pkText ? pkText->GetFontTexturePointer() : nullptr;

	STextAlignment align = ComputeAlignmentOffset();

	float fStanX = m_v3Position.x + align.offsetX;
	float fStanY = m_v3Position.y + align.offsetY + 1.0f;

	// 1. Warstwa zaznaczenia (Selection)
	int selBegin = 0;
	int selEnd = 0;
	if (m_isCursor && CIME::ms_bCaptureInput)
	{
		selBegin = CIME::GetSelBegin();
		selEnd = CIME::GetSelEnd();
	}
	else
	{
		selBegin = m_selStart;
		selEnd = m_selEnd;
	}

	if (selBegin > selEnd)
		std::swap(selBegin, selEnd);

	if (selBegin != selEnd)
	{
		float sx = 0.0f, sy = 0.0f, ex = 0.0f;
		int visualSelBegin = selBegin;
		int visualSelEnd = selEnd;
		if (!m_logicalToVisualPos.empty())
		{
			if (selBegin >= 0 && selBegin < (int)m_logicalToVisualPos.size())
				visualSelBegin = m_logicalToVisualPos[selBegin];
			if (selEnd >= 0 && selEnd < (int)m_logicalToVisualPos.size())
				visualSelEnd = m_logicalToVisualPos[selEnd];
		}

		__GetTextPos(visualSelBegin, &sx, &sy);
		__GetTextPos(visualSelEnd, &ex, &sy);

		sx += m_v3Position.x + align.offsetX;
		ex += m_v3Position.x + align.offsetX;

		float top = m_v3Position.y + align.offsetY;
		float bot = top + m_textHeight;

		STextQuad selQuad{};
		selQuad.hTexture = 0;
		selQuad.x = (sx < ex) ? sx : ex;
		selQuad.y = top;
		selQuad.w = (sx < ex) ? (ex - sx) : (sx - ex);
		selQuad.h = bot - top;
		selQuad.u0 = 0.0f;
		selQuad.v0 = 0.0f;
		selQuad.u1 = 1.0f;
		selQuad.v1 = 1.0f;
		selQuad.color = 0x80339CFF;
		selQuad.layer = ETextQuadLayer::Selection;
		m_layoutQuads.push_back(selQuad);
	}

	// 2. Warstwa obrysu (Outline)
	if (m_isOutline)
	{
		const float fFontHalfWeight = 1.0f;

		float fCurX = fStanX;
		float fCurY = fStanY;
		float fFontMaxHeight = 0.0f;

		for (size_t charIdx = 0; charIdx < m_pCharInfoVector.size(); ++charIdx)
		{
			CGraphicFontTexture::TCharacterInfomation* pCurCharInfo = m_pCharInfoVector[charIdx];
			if (!pCurCharInfo)
				continue;

			float fKern = (charIdx < m_kernVector.size()) ? m_kernVector[charIdx] : 0.0f;
			fCurX += fKern;

			float fFontWidth = float(pCurCharInfo->width);
			float fFontHeight = float(pCurCharInfo->height);
			fFontMaxHeight = (std::max)(fFontMaxHeight, fFontHeight);
			float fFontAdvance = float(pCurCharInfo->advance);

			if ((fCurX + fFontWidth) - m_v3Position.x > m_fLimitWidth) [[unlikely]]
			{
				if (m_isMultiLine)
				{
					fCurX = fStanX;
					fCurY += fFontMaxHeight;
				}
				else
				{
					break;
				}
			}

			if (pClipRect && fCurY <= pClipRect->top)
			{
				fCurX += fFontAdvance;
				continue;
			}

			float fFontSx = fCurX + pCurCharInfo->bearingX - 0.5f;
			float fFontSy = fCurY - 0.5f;

			uintptr_t hTexture = 0;
			if (pFontTexture && pCurCharInfo->index >= 0 && pFontTexture->CheckTextureIndex(pCurCharInfo->index))
			{
				pFontTexture->SelectTexture(pCurCharInfo->index);
				hTexture = reinterpret_cast<uintptr_t>(pFontTexture->GetD3DTexture());
			}

			const float offsets[4][2] = {
				{ -fFontHalfWeight, 0.0f },
				{  fFontHalfWeight, 0.0f },
				{ 0.0f, -fFontHalfWeight },
				{ 0.0f,  fFontHalfWeight }
			};

			for (int d = 0; d < 4; ++d)
			{
				STextQuad outlineQuad{};
				outlineQuad.hTexture = hTexture;
				outlineQuad.x = fFontSx + offsets[d][0];
				outlineQuad.y = fFontSy + offsets[d][1];
				outlineQuad.w = fFontWidth;
				outlineQuad.h = fFontHeight;
				outlineQuad.u0 = pCurCharInfo->left;
				outlineQuad.v0 = pCurCharInfo->top;
				outlineQuad.u1 = pCurCharInfo->right;
				outlineQuad.v1 = pCurCharInfo->bottom;
				outlineQuad.color = m_dwOutLineColor;
				outlineQuad.layer = ETextQuadLayer::Outline;
				m_layoutQuads.push_back(outlineQuad);
			}

			fCurX += fFontAdvance;
		}
	}

	// 3. Glowna warstwa tekstu (Main)
	{
		float fCurX = fStanX;
		float fCurY = fStanY;
		float fFontMaxHeight = 0.0f;

		for (size_t i = 0; i < m_pCharInfoVector.size(); ++i)
		{
			CGraphicFontTexture::TCharacterInfomation* pCurCharInfo = m_pCharInfoVector[i];
			if (!pCurCharInfo)
				continue;

			float fKern = (i < m_kernVector.size()) ? m_kernVector[i] : 0.0f;
			fCurX += fKern;

			float fFontWidth = float(pCurCharInfo->width);
			float fFontHeight = float(pCurCharInfo->height);
			fFontMaxHeight = (std::max)(fFontMaxHeight, fFontHeight);
			float fFontAdvance = float(pCurCharInfo->advance);

			if ((fCurX + fFontWidth) - m_v3Position.x > m_fLimitWidth) [[unlikely]]
			{
				if (m_isMultiLine)
				{
					fCurX = fStanX;
					fCurY += fFontMaxHeight;
				}
				else
				{
					break;
				}
			}

			if (pClipRect && fCurY <= pClipRect->top)
			{
				fCurX += fFontAdvance;
				continue;
			}

			float fFontSx = fCurX + pCurCharInfo->bearingX - 0.5f;
			float fFontSy = fCurY - 0.5f;

			uintptr_t hTexture = 0;
			if (pFontTexture && pCurCharInfo->index >= 0 && pFontTexture->CheckTextureIndex(pCurCharInfo->index))
			{
				pFontTexture->SelectTexture(pCurCharInfo->index);
				hTexture = reinterpret_cast<uintptr_t>(pFontTexture->GetD3DTexture());
			}

			STextQuad mainQuad{};
			mainQuad.hTexture = hTexture;
			mainQuad.x = fFontSx;
			mainQuad.y = fFontSy;
			mainQuad.w = fFontWidth;
			mainQuad.h = fFontHeight;
			mainQuad.u0 = pCurCharInfo->left;
			mainQuad.v0 = pCurCharInfo->top;
			mainQuad.u1 = pCurCharInfo->right;
			mainQuad.v1 = pCurCharInfo->bottom;
			mainQuad.color = (i < m_dwColorInfoVector.size()) ? m_dwColorInfoVector[i] : m_dwTextColor;
			mainQuad.layer = ETextQuadLayer::Main;
			m_layoutQuads.push_back(mainQuad);

			fCurX += fFontAdvance;
		}
	}

	// 4. Warstwa kursora (Cursor)
	if (m_isCursor)
	{
		float sx = 0.0f, sy = 0.0f, ex = 0.0f, ey = 0.0f;
		uint32_t diffuse = 0xFFFFFFFF;

		int curpos = CIME::GetCurPos();
		int compend = curpos + CIME::GetCompLen();

		int visualCurpos = curpos;
		int visualCompend = compend;
		if (!m_logicalToVisualPos.empty())
		{
			if (curpos >= 0 && curpos < (int)m_logicalToVisualPos.size())
				visualCurpos = m_logicalToVisualPos[curpos];
			if (compend >= 0 && compend < (int)m_logicalToVisualPos.size())
				visualCompend = m_logicalToVisualPos[compend];
		}

		__GetTextPos(visualCurpos, &sx, &sy);

		if (visualCurpos < visualCompend)
		{
			diffuse = 0x7FFFFFFF;
			__GetTextPos(visualCompend, &ex, &sy);
		}
		else
		{
			diffuse = 0xFFFFFFFF;
			ex = sx + 2.0f;
		}

		sx += m_v3Position.x + align.offsetX;
		ex += m_v3Position.x + align.offsetX;
		sy += m_v3Position.y + align.offsetY;
		ey = sy + m_textHeight;

		STextQuad cursorQuad{};
		cursorQuad.hTexture = 0;
		cursorQuad.x = (sx < ex) ? sx : ex;
		cursorQuad.y = sy;
		cursorQuad.w = (sx < ex) ? (ex - sx) : (sx - ex);
		cursorQuad.h = ey - sy;
		cursorQuad.u0 = 0.0f;
		cursorQuad.v0 = 0.0f;
		cursorQuad.u1 = 1.0f;
		cursorQuad.v1 = 1.0f;
		cursorQuad.color = diffuse;
		cursorQuad.layer = ETextQuadLayer::Cursor;
		m_layoutQuads.push_back(cursorQuad);

		// 5. Warstwa podkreslenia IME (Underline)
		int ulbegin = CIME::GetULBegin();
		int ulend = CIME::GetULEnd();
		if (ulbegin < ulend)
		{
			__GetTextPos(curpos + ulbegin, &sx, &sy);
			__GetTextPos(curpos + ulend, &ex, &sy);

			sx += m_v3Position.x + align.offsetX;
			ex += m_v3Position.x + align.offsetX;
			sy += m_v3Position.y + align.offsetY + m_textHeight;
			ey = sy + 2.0f;

			STextQuad ulQuad{};
			ulQuad.hTexture = 0;
			ulQuad.x = (sx < ex) ? sx : ex;
			ulQuad.y = sy;
			ulQuad.w = (sx < ex) ? (ex - sx) : (sx - ex);
			ulQuad.h = ey - sy;
			ulQuad.u0 = 0.0f;
			ulQuad.v0 = 0.0f;
			ulQuad.u1 = 1.0f;
			ulQuad.v1 = 1.0f;
			ulQuad.color = 0xFFFF0000;
			ulQuad.layer = ETextQuadLayer::Underline;
			m_layoutQuads.push_back(ulQuad);
		}
	}

	return m_layoutQuads;
}

void CGraphicTextInstance::Render(RECT * pClipRect)
{
	if (!m_isUpdate)
		return;

	CGraphicText* pkText = m_roText.GetPointer();
	if (!pkText)
		return;

	CGraphicFontTexture* pFontTexture = pkText->GetFontTexturePointer();
	if (!pFontTexture)
		return;

	const std::vector<STextQuad>& quads = LayoutGlyphs(pClipRect);

	// 1. Wysylanie wygenerowanych quadow bezposrednio do CDynamicQuadBatcher
	auto& batcher = CDynamicQuadBatcher::Instance();
	for (const auto& quad : quads)
	{
		batcher.AddQuad(quad.x, quad.y, quad.w, quad.h, quad.color, quad.hTexture,
		                quad.u0, quad.v0, quad.u1, quad.v1);
	}

	// 2. Bezposrednie draw calle zabezpieczone bramka LegacyDraws(RenderOwnership::UI)
	if (RenderOwnership::LegacyDraws(RenderOwnership::ERenderCategory::UI))
	{
		for (auto& pair : m_outlineBatches)
			pair.second.clear();
		for (auto& pair : m_mainBatches)
			pair.second.clear();

		STATEMANAGER.SaveRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
		STATEMANAGER.SaveRenderState(D3DRS_SRCBLEND, D3DBLEND_SRCALPHA);
		STATEMANAGER.SaveRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCALPHA);
		STATEMANAGER.SaveRenderState(D3DRS_ZENABLE, D3DZB_FALSE);
		STATEMANAGER.SaveRenderState(D3DRS_CULLMODE, D3DCULL_NONE);

		DWORD dwFogEnable = STATEMANAGER.GetRenderState(D3DRS_FOGENABLE);
		DWORD dwLighting = STATEMANAGER.GetRenderState(D3DRS_LIGHTING);
		STATEMANAGER.SetRenderState(D3DRS_FOGENABLE, FALSE);
		STATEMANAGER.SetRenderState(D3DRS_LIGHTING, FALSE);
		STATEMANAGER.SetRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
		STATEMANAGER.SetRenderState(D3DRS_SRCBLEND, D3DBLEND_SRCALPHA);
		STATEMANAGER.SetRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCALPHA);
		STATEMANAGER.SetRenderState(D3DRS_ZENABLE, D3DZB_FALSE);
		STATEMANAGER.SetRenderState(D3DRS_CULLMODE, D3DCULL_NONE);

		STATEMANAGER.SetFVF(D3DFVF_XYZ|D3DFVF_DIFFUSE|D3DFVF_TEX1);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG1,	D3DTA_TEXTURE);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG2,	D3DTA_DIFFUSE);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_COLOROP,	D3DTOP_MODULATE);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAARG1,	D3DTA_TEXTURE);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAARG2,	D3DTA_DIFFUSE);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAOP,	D3DTOP_MODULATE);

		SVertex akVertex[4];
		akVertex[0].z = m_v3Position.z;
		akVertex[1].z = m_v3Position.z;
		akVertex[2].z = m_v3Position.z;
		akVertex[3].z = m_v3Position.z;

		for (const auto& quad : quads)
		{
			if (quad.layer == ETextQuadLayer::Outline || quad.layer == ETextQuadLayer::Main)
			{
				LPDIRECT3DTEXTURE9 pTex = reinterpret_cast<LPDIRECT3DTEXTURE9>(quad.hTexture);
				auto& vtxBatch = (quad.layer == ETextQuadLayer::Outline) ? m_outlineBatches[pTex] : m_mainBatches[pTex];

				akVertex[0].x = quad.x;          akVertex[0].y = quad.y;          akVertex[0].u = quad.u0; akVertex[0].v = quad.v0; akVertex[0].color = quad.color;
				akVertex[1].x = quad.x;          akVertex[1].y = quad.y + quad.h; akVertex[1].u = quad.u0; akVertex[1].v = quad.v1; akVertex[1].color = quad.color;
				akVertex[2].x = quad.x + quad.w; akVertex[2].y = quad.y;          akVertex[2].u = quad.u1; akVertex[2].v = quad.v0; akVertex[2].color = quad.color;
				akVertex[3].x = quad.x + quad.w; akVertex[3].y = quad.y + quad.h; akVertex[3].u = quad.u1; akVertex[3].v = quad.v1; akVertex[3].color = quad.color;

				vtxBatch.push_back(akVertex[0]); vtxBatch.push_back(akVertex[1]); vtxBatch.push_back(akVertex[2]);
				vtxBatch.push_back(akVertex[2]); vtxBatch.push_back(akVertex[1]); vtxBatch.push_back(akVertex[3]);
			}
			else if (quad.layer == ETextQuadLayer::Selection || quad.layer == ETextQuadLayer::Cursor || quad.layer == ETextQuadLayer::Underline)
			{
				TPDTVertex vertices[4];
				vertices[0].diffuse = quad.color;
				vertices[1].diffuse = quad.color;
				vertices[2].diffuse = quad.color;
				vertices[3].diffuse = quad.color;

				vertices[0].position = TPosition(quad.x, quad.y, 0.0f);
				vertices[1].position = TPosition(quad.x + quad.w, quad.y, 0.0f);
				vertices[2].position = TPosition(quad.x, quad.y + quad.h, 0.0f);
				vertices[3].position = TPosition(quad.x + quad.w, quad.y + quad.h, 0.0f);

				STATEMANAGER.SetTexture(0, NULL);
				CGraphicBase::SetDefaultIndexBuffer(CGraphicBase::DEFAULT_IB_FILL_RECT);
				if (CGraphicBase::SetPDTStream(vertices, 4))
					STATEMANAGER.DrawIndexedPrimitive(D3DPT_TRIANGLELIST, 0, 4, 0, 2);
			}
		}

		auto DrawBatch = [](const std::unordered_map<LPDIRECT3DTEXTURE9, std::vector<SVertex>>& batches) {
			for (const auto& [pTexture, vtxBatch] : batches) {
				if (vtxBatch.empty())
					continue;

				STATEMANAGER.SetTexture(0, pTexture);
				STATEMANAGER.DrawPrimitiveUP(D3DPT_TRIANGLELIST,
					vtxBatch.size() / 3, vtxBatch.data(), sizeof(SVertex));
			}
		};

		DrawBatch(m_outlineBatches);
		DrawBatch(m_mainBatches);

		STATEMANAGER.RestoreRenderState(D3DRS_CULLMODE);
		STATEMANAGER.RestoreRenderState(D3DRS_ZENABLE);
		STATEMANAGER.RestoreRenderState(D3DRS_DESTBLEND);
		STATEMANAGER.RestoreRenderState(D3DRS_SRCBLEND);
		STATEMANAGER.RestoreRenderState(D3DRS_ALPHABLENDENABLE);

		STATEMANAGER.SetRenderState(D3DRS_FOGENABLE, dwFogEnable);
		STATEMANAGER.SetRenderState(D3DRS_LIGHTING, dwLighting);
	}

	if (m_hyperlinkVector.size() != 0)
	{
		STextAlignment align = ComputeAlignmentOffset();
		int textOffsetX = (int)(m_v3Position.x + align.offsetX);
		int textOffsetY = (int)(m_v3Position.y + align.offsetY);

		int lx = gs_mx - textOffsetX;
		int ly = gs_my - textOffsetY;

		if (lx >= 0 && ly >= 0 && lx < m_textWidth && ly < m_textHeight)
		{
			for (auto& link : m_hyperlinkVector)
			{
				if (lx >= link.sx && lx < link.ex)
				{
					gs_hyperlinkText = link.text;
					break;
				}
			}
		}
	}
}

void CGraphicTextInstance::CreateSystem(UINT uCapacity)
{
	ms_kPool.Create(uCapacity);
}

void CGraphicTextInstance::DestroySystem()
{
	ms_kPool.Destroy();
}

CGraphicTextInstance* CGraphicTextInstance::New()
{
	return ms_kPool.Alloc();
}

void CGraphicTextInstance::Delete(CGraphicTextInstance* pkInst)
{
	pkInst->Destroy();
	ms_kPool.Free(pkInst);
}

void CGraphicTextInstance::ShowCursor()
{
	m_isCursor = true;
}

void CGraphicTextInstance::HideCursor()
{
	m_isCursor = false;
}

void CGraphicTextInstance::SetSelection(int iStart, int iEnd)
{
	m_selStart = iStart;
	m_selEnd = iEnd;
}

void CGraphicTextInstance::ClearSelection()
{
	m_selStart = 0;
	m_selEnd = 0;
}

void CGraphicTextInstance::ShowOutLine()
{
	m_isOutline = true;
}

void CGraphicTextInstance::HideOutLine()
{
	m_isOutline = false;
}

void CGraphicTextInstance::SetColor(DWORD color)
{
	if (m_dwTextColor != color)
	{
		for (int i = 0; i < m_pCharInfoVector.size(); ++i)
			if (m_dwColorInfoVector[i] == m_dwTextColor)
				m_dwColorInfoVector[i] = color;

		m_dwTextColor = color;
	}
}

void CGraphicTextInstance::SetColor(float r, float g, float b, float a)
{
	SetColor(D3DXCOLOR(r, g, b, a));
}

void CGraphicTextInstance::SetOutLineColor(DWORD color)
{
	m_dwOutLineColor=color;
}

void CGraphicTextInstance::SetOutLineColor(float r, float g, float b, float a)
{
	m_dwOutLineColor=D3DXCOLOR(r, g, b, a);
}

void CGraphicTextInstance::SetSecret(bool Value)
{
	m_isSecret = Value;
}

void CGraphicTextInstance::SetOutline(bool Value)
{
	m_isOutline = Value;
}

void CGraphicTextInstance::SetFeather(bool Value)
{
	if (Value)
	{
		m_fFontFeather = c_fFontFeather;
	}
	else
	{
		m_fFontFeather = 0.0f;
	}
}

void CGraphicTextInstance::SetMultiLine(bool Value)
{
	m_isMultiLine = Value;
}

void CGraphicTextInstance::SetHorizontalAlign(int iType)
{
	m_hAlign = iType;
}

void CGraphicTextInstance::SetVerticalAlign(int vAlign)
{
	m_vAlign = vAlign;
}

void CGraphicTextInstance::SetMax(int iMax)
{
	m_iMax = iMax;
}

void CGraphicTextInstance::SetLimitWidth(float fWidth)
{
	m_fLimitWidth = fWidth;
}

void CGraphicTextInstance::SetValueString(const std::string& c_stValue)
{
	if (0 == m_stText.compare(c_stValue))
		return;

	m_stText = c_stValue;
	m_isUpdate = false;
}

void CGraphicTextInstance::SetValue(const char* c_szText, size_t len)
{
	if (0 == m_stText.compare(c_szText))
		return;

	m_stText = c_szText;
	m_isChatMessage = false; // Reset chat mode
	m_isUpdate = false;
}

void CGraphicTextInstance::SetChatValue(const char* c_szName, const char* c_szMessage)
{
	if (!c_szName || !c_szMessage)
		return;

	// Store separated components
	m_chatName = c_szName;
	m_chatMessage = c_szMessage;
	m_isChatMessage = true;

	// Build combined text for rendering (will be processed by Update())
	// Use BuildVisualChatMessage in Update() instead of BuildVisualBidiText_Tagless
	m_stText = std::string(c_szName) + " : " + std::string(c_szMessage);
	m_isUpdate = false;
}

void CGraphicTextInstance::SetPosition(float fx, float fy, float fz)
{
	m_v3Position.x = fx;
	m_v3Position.y = fy;
	m_v3Position.z = fz;
}

void CGraphicTextInstance::SetTextPointer(CGraphicText* pText)
{
	m_roText = pText;
}

void CGraphicTextInstance::SetTextDirection(ETextDirection dir)
{
	if (m_direction == dir)
		return;

	m_direction = dir;
	m_isUpdate = false;
}

const std::string & CGraphicTextInstance::GetValueStringReference()
{
	return m_stText;
}

WORD CGraphicTextInstance::GetTextLineCount()
{
	CGraphicFontTexture::TCharacterInfomation* pCurCharInfo;
	CGraphicFontTexture::TPCharacterInfomationVector::iterator itor;

	float fx = 0.0f;
	WORD wLineCount = 1;
	for (itor=m_pCharInfoVector.begin(); itor!=m_pCharInfoVector.end(); ++itor)
	{
		pCurCharInfo = *itor;

		float fFontWidth=float(pCurCharInfo->width);
		float fFontAdvance=float(pCurCharInfo->advance);
		//float fFontHeight=float(pCurCharInfo->height);

		// NOTE : 폰트 출력에 Width 제한을 둡니다. - [levites]
		if (fx+fFontWidth > m_fLimitWidth)
		{
			fx = 0.0f;
			++wLineCount;
		}

		fx += fFontAdvance;
	}

	return wLineCount;
}

void CGraphicTextInstance::GetTextSize(int* pRetWidth, int* pRetHeight)
{
	*pRetWidth = m_textWidth;
	*pRetHeight = m_textHeight;
}

int CGraphicTextInstance::PixelPositionToCharacterPosition(int iPixelPosition)
{
	// Clamp to valid range [0, textWidth]
	int adjustedPixelPos = iPixelPosition;
	if (adjustedPixelPos < 0)
		adjustedPixelPos = 0;
	if (adjustedPixelPos > m_textWidth)
		adjustedPixelPos = m_textWidth;

	// RTL: interpret click from right edge of rendered text
	if (m_computedRTL)
		adjustedPixelPos = m_textWidth - adjustedPixelPos;

	int icurPosition = 0;
	int visualPos = -1;

	for (int i = 0; i < (int)m_pCharInfoVector.size(); ++i)
	{
		CGraphicFontTexture::TCharacterInfomation* pCurCharInfo = m_pCharInfoVector[i];

		// Use advance instead of width (width is not reliable for cursor hit-testing)
		int adv = pCurCharInfo->advance;
		if (adv <= 0)
			adv = pCurCharInfo->width;

		int charStart = icurPosition;
		icurPosition += adv;

		int charMid = charStart + adv / 2;

		if (adjustedPixelPos < charMid)
		{
			visualPos = i;
			break;
		}
	}

	if (visualPos < 0)
		visualPos = (int)m_pCharInfoVector.size();

	if (!m_visualToLogicalPos.empty() && visualPos >= 0 && visualPos < (int)m_visualToLogicalPos.size())
		return m_visualToLogicalPos[visualPos];

	return visualPos;
}

int CGraphicTextInstance::GetHorizontalAlign()
{
	return m_hAlign;
}

void CGraphicTextInstance::__Initialize()
{
	m_roText = NULL;

	m_hAlign = HORIZONTAL_ALIGN_LEFT;
	m_vAlign = VERTICAL_ALIGN_TOP;

	m_iMax = 0;
	m_fLimitWidth = 1600.0f;

	m_isCursor = false;
	m_isSecret = false;
	m_isMultiLine = false;

	m_selStart = 0;
	m_selEnd = 0;

	m_isOutline = false;
	m_fFontFeather = c_fFontFeather;

	m_isUpdate = false;
	// Use Auto direction for input fields - they should auto-detect based on content
	// Only chat messages should be explicitly set to RTL
	m_direction = ETextDirection::Auto;
	m_computedRTL = false;
	m_isChatMessage = false;
	m_chatName = "";
	m_chatMessage = "";

	m_textWidth = 0;
	m_textHeight = 0;

	m_v3Position.x = m_v3Position.y = m_v3Position.z = 0.0f;

	m_dwOutLineColor=0xff000000;

	m_layoutQuads.clear();
	m_wTextBuf.clear();
	m_currentSegment.clear();
	m_newCharInfos.clear();
	m_newColors.clear();
	m_newKerns.clear();
	m_visibleToRender.clear();
	m_content.clear();
	m_outlineBatches.clear();
	m_mainBatches.clear();
}

void CGraphicTextInstance::Destroy()
{
	m_stText="";
	m_pCharInfoVector.clear();
	m_kernVector.clear();
	m_dwColorInfoVector.clear();
	m_hyperlinkVector.clear();
	m_logicalToVisualPos.clear();
	m_visualToLogicalPos.clear();
	m_layoutQuads.clear();
	m_wTextBuf.clear();
	m_currentSegment.clear();
	m_newCharInfos.clear();
	m_newColors.clear();
	m_newKerns.clear();
	m_visibleToRender.clear();
	m_content.clear();
	m_outlineBatches.clear();
	m_mainBatches.clear();

	__Initialize();
}

CGraphicTextInstance::CGraphicTextInstance()
{
	__Initialize();
}

CGraphicTextInstance::~CGraphicTextInstance()
{
	Destroy();
}
