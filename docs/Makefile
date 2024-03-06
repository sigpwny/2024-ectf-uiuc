default:
	@echo "Building the Design Document"
	@pandoc -F mermaid-filter -t markdown \
	1-overview.md \
	newpage.tex \
	2-hide_protocol.md \
	newpage.tex \
	3-misc_protocol.md \
	newpage.tex \
	4-security_requirements.md \
	| pandoc --number-sections -V geometry:margin=1.5in -F pandoc-crossref --pdf-engine=tectonic -o 2024_eCTF_UIUC_Design_Document.pdf