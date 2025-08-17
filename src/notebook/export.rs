//! # 笔记本导出功能
//!
//! 提供将笔记本导出为不同格式的功能。

use super::{Notebook, NotebookCell, CellType, NotebookError, NotebookResult};
use crate::formatter::{FormatType, FormatOptions};
use std::fs;
use std::path::Path;

/// 导出格式
#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
    /// HTML 格式（包含数学公式渲染）
    Html,
    /// PDF 格式（通过 LaTeX）
    Pdf,
    /// Markdown 格式
    Markdown,
    /// 纯代码文件
    Code,
    /// LaTeX 格式
    Latex,
}

impl ExportFormat {
    /// 获取格式的文件扩展名
    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::Html => "html",
            ExportFormat::Pdf => "pdf",
            ExportFormat::Markdown => "md",
            ExportFormat::Code => "txt",
            ExportFormat::Latex => "tex",
        }
    }
    
    /// 获取格式的 MIME 类型
    pub fn mime_type(&self) -> &'static str {
        match self {
            ExportFormat::Html => "text/html",
            ExportFormat::Pdf => "application/pdf",
            ExportFormat::Markdown => "text/markdown",
            ExportFormat::Code => "text/plain",
            ExportFormat::Latex => "application/x-latex",
        }
    }
    
    /// 获取格式的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            ExportFormat::Html => "HTML",
            ExportFormat::Pdf => "PDF",
            ExportFormat::Markdown => "Markdown",
            ExportFormat::Code => "代码",
            ExportFormat::Latex => "LaTeX",
        }
    }
}

/// 导出选项
#[derive(Debug, Clone)]
pub struct ExportOptions {
    /// 是否包含输出结果
    pub include_outputs: bool,
    /// 是否包含元数据
    pub include_metadata: bool,
    /// 是否包含执行时间
    pub include_timing: bool,
    /// 数学公式渲染方式
    pub math_renderer: MathRenderer,
    /// 代码语法高亮
    pub syntax_highlighting: bool,
    /// 自定义 CSS（仅用于 HTML）
    pub custom_css: Option<String>,
    /// 自定义模板
    pub custom_template: Option<String>,
    /// 页面标题
    pub page_title: Option<String>,
    /// 作者信息
    pub author: Option<String>,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            include_outputs: true,
            include_metadata: true,
            include_timing: false,
            math_renderer: MathRenderer::MathJax,
            syntax_highlighting: true,
            custom_css: None,
            custom_template: None,
            page_title: None,
            author: None,
        }
    }
}

/// 数学公式渲染器
#[derive(Debug, Clone, PartialEq)]
pub enum MathRenderer {
    /// MathJax
    MathJax,
    /// KaTeX
    KaTeX,
    /// 纯文本
    PlainText,
}

/// 笔记本导出器
pub struct NotebookExporter {
    /// 导出选项
    options: ExportOptions,
}

impl NotebookExporter {
    /// 创建新的导出器
    pub fn new() -> Self {
        Self {
            options: ExportOptions::default(),
        }
    }
    
    /// 使用指定选项创建导出器
    pub fn with_options(options: ExportOptions) -> Self {
        Self { options }
    }
    
    /// 设置导出选项
    pub fn set_options(&mut self, options: ExportOptions) {
        self.options = options;
    }
    
    /// 导出笔记本到文件
    pub fn export_to_file<P: AsRef<Path>>(
        &self,
        notebook: &Notebook,
        path: P,
        format: ExportFormat,
    ) -> NotebookResult<()> {
        let content = self.export_to_string(notebook, format.clone())?;
        
        let path = path.as_ref();
        
        // 确保文件扩展名正确
        let expected_ext = format.extension();
        if path.extension().and_then(|s| s.to_str()) != Some(expected_ext) {
            return Err(NotebookError::Format(
                format!("文件扩展名应该是 .{}", expected_ext)
            ));
        }
        
        fs::write(path, content)
            .map_err(|e| NotebookError::Io(e))?;
        
        Ok(())
    }
    
    /// 导出笔记本为字符串
    pub fn export_to_string(&self, notebook: &Notebook, format: ExportFormat) -> NotebookResult<String> {
        match format {
            ExportFormat::Html => self.export_to_html(notebook),
            ExportFormat::Pdf => self.export_to_pdf(notebook),
            ExportFormat::Markdown => self.export_to_markdown(notebook),
            ExportFormat::Code => self.export_to_code(notebook),
            ExportFormat::Latex => self.export_to_latex(notebook),
        }
    }
    
    /// 导出为 HTML
    fn export_to_html(&self, notebook: &Notebook) -> NotebookResult<String> {
        let mut html = String::new();
        
        // HTML 头部
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"zh-CN\">\n");
        html.push_str("<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        
        let title = self.options.page_title.as_deref()
            .unwrap_or(&notebook.metadata.title);
        html.push_str(&format!("    <title>{}</title>\n", title));
        
        // 添加数学公式渲染支持
        match self.options.math_renderer {
            MathRenderer::MathJax => {
                html.push_str("    <script src=\"https://polyfill.io/v3/polyfill.min.js?features=es6\"></script>\n");
                html.push_str("    <script id=\"MathJax-script\" async src=\"https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js\"></script>\n");
            }
            MathRenderer::KaTeX => {
                html.push_str("    <link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/katex@0.16.0/dist/katex.min.css\">\n");
                html.push_str("    <script defer src=\"https://cdn.jsdelivr.net/npm/katex@0.16.0/dist/katex.min.js\"></script>\n");
                html.push_str("    <script defer src=\"https://cdn.jsdelivr.net/npm/katex@0.16.0/dist/contrib/auto-render.min.js\"></script>\n");
            }
            MathRenderer::PlainText => {}
        }
        
        // 添加样式
        html.push_str("    <style>\n");
        html.push_str(self.get_default_css());
        if let Some(custom_css) = &self.options.custom_css {
            html.push_str(custom_css);
        }
        html.push_str("    </style>\n");
        
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        
        // 笔记本标题和元数据
        if self.options.include_metadata {
            html.push_str(&format!("    <h1>{}</h1>\n", notebook.metadata.title));
            
            if let Some(author) = &notebook.metadata.author {
                html.push_str(&format!("    <p class=\"author\">作者: {}</p>\n", author));
            }
            
            if let Some(description) = &notebook.metadata.description {
                html.push_str(&format!("    <p class=\"description\">{}</p>\n", description));
            }
        }
        
        // 单元格内容
        html.push_str("    <div class=\"notebook-content\">\n");
        
        for (index, cell) in notebook.cells.iter().enumerate() {
            html.push_str(&self.render_cell_html(cell, index)?);
        }
        
        html.push_str("    </div>\n");
        
        // 添加脚本
        if self.options.math_renderer == MathRenderer::KaTeX {
            html.push_str("    <script>\n");
            html.push_str("        document.addEventListener(\"DOMContentLoaded\", function() {\n");
            html.push_str("            renderMathInElement(document.body);\n");
            html.push_str("        });\n");
            html.push_str("    </script>\n");
        }
        
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        Ok(html)
    }
    
    /// 渲染单元格为 HTML
    fn render_cell_html(&self, cell: &NotebookCell, index: usize) -> NotebookResult<String> {
        let mut html = String::new();
        
        let cell_class = match cell.cell_type {
            CellType::Code => "code-cell",
            CellType::Text => "text-cell",
            CellType::Markdown => "markdown-cell",
            CellType::Output => "output-cell",
        };
        
        html.push_str(&format!("        <div class=\"cell {}\" data-cell-index=\"{}\">\n", cell_class, index));
        
        // 单元格标签
        if self.options.include_metadata {
            html.push_str(&format!("            <div class=\"cell-label\">[{}] {}</div>\n", 
                                 index + 1, cell.cell_type.display_name()));
        }
        
        // 单元格内容
        match cell.cell_type {
            CellType::Code => {
                html.push_str("            <div class=\"cell-input\">\n");
                if self.options.syntax_highlighting {
                    html.push_str("                <pre><code class=\"language-yufmath\">");
                } else {
                    html.push_str("                <pre><code>");
                }
                html.push_str(&self.escape_html(&cell.get_text()));
                html.push_str("</code></pre>\n");
                html.push_str("            </div>\n");
                
                // 输出结果
                if self.options.include_outputs {
                    if let Some(output) = cell.get_output() {
                        html.push_str("            <div class=\"cell-output\">\n");
                        html.push_str("                <pre>");
                        html.push_str(&self.escape_html(&output.get_text()));
                        html.push_str("</pre>\n");
                        html.push_str("            </div>\n");
                    }
                }
            }
            CellType::Markdown => {
                html.push_str("            <div class=\"cell-content\">\n");
                html.push_str(&self.render_markdown(&cell.get_text()));
                html.push_str("            </div>\n");
            }
            CellType::Text => {
                html.push_str("            <div class=\"cell-content\">\n");
                html.push_str("                <p>");
                html.push_str(&self.escape_html(&cell.get_text()));
                html.push_str("</p>\n");
                html.push_str("            </div>\n");
            }
            CellType::Output => {
                // 输出单元格通常不单独渲染
            }
        }
        
        html.push_str("        </div>\n");
        
        Ok(html)
    }
    
    /// 导出为 Markdown
    fn export_to_markdown(&self, notebook: &Notebook) -> NotebookResult<String> {
        let mut md = String::new();
        
        // 标题和元数据
        if self.options.include_metadata {
            md.push_str(&format!("# {}\n\n", notebook.metadata.title));
            
            if let Some(author) = &notebook.metadata.author {
                md.push_str(&format!("**作者**: {}\n\n", author));
            }
            
            if let Some(description) = &notebook.metadata.description {
                md.push_str(&format!("{}\n\n", description));
            }
        }
        
        // 单元格内容
        for (index, cell) in notebook.cells.iter().enumerate() {
            match cell.cell_type {
                CellType::Code => {
                    if self.options.include_metadata {
                        md.push_str(&format!("## 代码单元格 {}\n\n", index + 1));
                    }
                    
                    md.push_str("```yufmath\n");
                    md.push_str(&cell.get_text());
                    md.push_str("\n```\n\n");
                    
                    if self.options.include_outputs {
                        if let Some(output) = cell.get_output() {
                            md.push_str("**输出**:\n\n");
                            md.push_str("```\n");
                            md.push_str(&output.get_text());
                            md.push_str("\n```\n\n");
                        }
                    }
                }
                CellType::Markdown => {
                    md.push_str(&cell.get_text());
                    md.push_str("\n\n");
                }
                CellType::Text => {
                    md.push_str(&cell.get_text());
                    md.push_str("\n\n");
                }
                CellType::Output => {
                    // 输出单元格通常不单独导出
                }
            }
        }
        
        Ok(md)
    }
    
    /// 导出为纯代码
    fn export_to_code(&self, notebook: &Notebook) -> NotebookResult<String> {
        let mut code = String::new();
        
        if self.options.include_metadata {
            code.push_str(&format!("# {}\n", notebook.metadata.title));
            if let Some(author) = &notebook.metadata.author {
                code.push_str(&format!("# 作者: {}\n", author));
            }
            code.push_str("\n");
        }
        
        for (index, cell) in notebook.cells.iter().enumerate() {
            if cell.cell_type == CellType::Code {
                if self.options.include_metadata {
                    code.push_str(&format!("# 单元格 {}\n", index + 1));
                }
                
                code.push_str(&cell.get_text());
                code.push_str("\n\n");
                
                if self.options.include_outputs {
                    if let Some(output) = cell.get_output() {
                        code.push_str(&format!("# 输出: {}\n\n", output.get_text()));
                    }
                }
            }
        }
        
        Ok(code)
    }
    
    /// 导出为 LaTeX
    fn export_to_latex(&self, notebook: &Notebook) -> NotebookResult<String> {
        let mut latex = String::new();
        
        // LaTeX 文档头部
        latex.push_str("\\documentclass{article}\n");
        latex.push_str("\\usepackage[utf8]{inputenc}\n");
        latex.push_str("\\usepackage{amsmath}\n");
        latex.push_str("\\usepackage{amsfonts}\n");
        latex.push_str("\\usepackage{amssymb}\n");
        latex.push_str("\\usepackage{listings}\n");
        latex.push_str("\\usepackage{xcolor}\n");
        latex.push_str("\\usepackage{geometry}\n");
        latex.push_str("\\geometry{a4paper,margin=1in}\n\n");
        
        // 代码样式设置
        latex.push_str("\\lstset{\n");
        latex.push_str("    basicstyle=\\ttfamily,\n");
        latex.push_str("    breaklines=true,\n");
        latex.push_str("    frame=single,\n");
        latex.push_str("    backgroundcolor=\\color{gray!10}\n");
        latex.push_str("}\n\n");
        
        latex.push_str("\\begin{document}\n\n");
        
        // 标题
        if self.options.include_metadata {
            latex.push_str(&format!("\\title{{{}}}\n", self.escape_latex(&notebook.metadata.title)));
            
            if let Some(author) = &notebook.metadata.author {
                latex.push_str(&format!("\\author{{{}}}\n", self.escape_latex(author)));
            }
            
            latex.push_str("\\maketitle\n\n");
            
            if let Some(description) = &notebook.metadata.description {
                latex.push_str(&format!("{}\n\n", self.escape_latex(description)));
            }
        }
        
        // 单元格内容
        for (index, cell) in notebook.cells.iter().enumerate() {
            match cell.cell_type {
                CellType::Code => {
                    if self.options.include_metadata {
                        latex.push_str(&format!("\\section{{代码单元格 {}}}\n\n", index + 1));
                    }
                    
                    latex.push_str("\\begin{lstlisting}\n");
                    latex.push_str(&cell.get_text());
                    latex.push_str("\n\\end{lstlisting}\n\n");
                    
                    if self.options.include_outputs {
                        if let Some(output) = cell.get_output() {
                            latex.push_str("\\textbf{输出:}\n\n");
                            latex.push_str("\\begin{verbatim}\n");
                            latex.push_str(&output.get_text());
                            latex.push_str("\n\\end{verbatim}\n\n");
                        }
                    }
                }
                CellType::Markdown => {
                    latex.push_str(&self.convert_markdown_to_latex(&cell.get_text()));
                    latex.push_str("\n\n");
                }
                CellType::Text => {
                    latex.push_str(&self.escape_latex(&cell.get_text()));
                    latex.push_str("\n\n");
                }
                CellType::Output => {}
            }
        }
        
        latex.push_str("\\end{document}\n");
        
        Ok(latex)
    }
    
    /// 导出为 PDF（通过 LaTeX）
    fn export_to_pdf(&self, notebook: &Notebook) -> NotebookResult<String> {
        // 这里应该调用 LaTeX 编译器生成 PDF
        // 由于这需要外部依赖，这里只返回 LaTeX 源码
        self.export_to_latex(notebook)
    }
    
    /// 获取默认 CSS
    fn get_default_css(&self) -> &'static str {
        r#"
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            line-height: 1.6;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            background-color: #fff;
        }
        
        .notebook-content {
            margin-top: 20px;
        }
        
        .cell {
            margin-bottom: 20px;
            border: 1px solid #e1e4e8;
            border-radius: 6px;
            overflow: hidden;
        }
        
        .cell-label {
            background-color: #f6f8fa;
            padding: 8px 12px;
            font-size: 12px;
            color: #586069;
            border-bottom: 1px solid #e1e4e8;
        }
        
        .cell-input, .cell-output {
            padding: 12px;
        }
        
        .cell-output {
            background-color: #f8f9fa;
            border-top: 1px solid #e1e4e8;
        }
        
        pre {
            margin: 0;
            overflow-x: auto;
        }
        
        code {
            font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
            font-size: 14px;
        }
        
        .author {
            color: #586069;
            font-style: italic;
        }
        
        .description {
            color: #24292e;
            margin-bottom: 30px;
        }
        "#
    }
    
    /// 转义 HTML 字符
    fn escape_html(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }
    
    /// 转义 LaTeX 字符
    fn escape_latex(&self, text: &str) -> String {
        text.replace('\\', "\\textbackslash{}")
            .replace('{', "\\{")
            .replace('}', "\\}")
            .replace('$', "\\$")
            .replace('&', "\\&")
            .replace('%', "\\%")
            .replace('#', "\\#")
            .replace('^', "\\textasciicircum{}")
            .replace('_', "\\_")
            .replace('~', "\\textasciitilde{}")
    }
    
    /// 简单的 Markdown 渲染
    fn render_markdown(&self, text: &str) -> String {
        let mut html = String::new();
        
        for line in text.lines() {
            if line.starts_with("# ") {
                html.push_str(&format!("                <h1>{}</h1>\n", &line[2..]));
            } else if line.starts_with("## ") {
                html.push_str(&format!("                <h2>{}</h2>\n", &line[3..]));
            } else if line.starts_with("### ") {
                html.push_str(&format!("                <h3>{}</h3>\n", &line[4..]));
            } else if line.trim().is_empty() {
                html.push_str("                <br>\n");
            } else {
                html.push_str(&format!("                <p>{}</p>\n", self.escape_html(line)));
            }
        }
        
        html
    }
    
    /// 将 Markdown 转换为 LaTeX
    fn convert_markdown_to_latex(&self, text: &str) -> String {
        let mut latex = String::new();
        
        for line in text.lines() {
            if line.starts_with("# ") {
                latex.push_str(&format!("\\section{{{}}}\n", self.escape_latex(&line[2..])));
            } else if line.starts_with("## ") {
                latex.push_str(&format!("\\subsection{{{}}}\n", self.escape_latex(&line[3..])));
            } else if line.starts_with("### ") {
                latex.push_str(&format!("\\subsubsection{{{}}}\n", self.escape_latex(&line[4..])));
            } else if !line.trim().is_empty() {
                latex.push_str(&format!("{}\n", self.escape_latex(line)));
            }
        }
        
        latex
    }
}

impl Default for NotebookExporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_export_format() {
        assert_eq!(ExportFormat::Html.extension(), "html");
        assert_eq!(ExportFormat::Html.mime_type(), "text/html");
        assert_eq!(ExportFormat::Html.display_name(), "HTML");
        
        assert_eq!(ExportFormat::Markdown.extension(), "md");
        assert_eq!(ExportFormat::Pdf.extension(), "pdf");
    }
    
    #[test]
    fn test_export_options() {
        let options = ExportOptions::default();
        assert!(options.include_outputs);
        assert!(options.include_metadata);
        assert!(!options.include_timing);
        assert_eq!(options.math_renderer, MathRenderer::MathJax);
    }
    
    #[test]
    fn test_notebook_exporter() {
        let exporter = NotebookExporter::new();
        let mut notebook = Notebook::with_title("测试笔记本".to_string());
        
        // 添加测试单元格
        notebook.add_cell(crate::NotebookCell::new_markdown("# 标题\n\n这是一个测试。".to_string()));
        notebook.add_cell(crate::NotebookCell::new_code("2 + 3".to_string()));
        
        // 测试导出为 Markdown
        let markdown = exporter.export_to_string(&notebook, ExportFormat::Markdown).unwrap();
        assert!(markdown.contains("# 测试笔记本"));
        assert!(markdown.contains("# 标题"));
        assert!(markdown.contains("2 + 3"));
        
        // 测试导出为代码
        let code = exporter.export_to_string(&notebook, ExportFormat::Code).unwrap();
        assert!(code.contains("2 + 3"));
        assert!(code.contains("# 测试笔记本"));
    }
    
    #[test]
    fn test_html_export() {
        let exporter = NotebookExporter::new();
        let mut notebook = Notebook::with_title("HTML 测试".to_string());
        notebook.add_cell(crate::NotebookCell::new_code("x = 42".to_string()));
        
        let html = exporter.export_to_html(&notebook).unwrap();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("HTML 测试"));
        assert!(html.contains("x = 42"));
        assert!(html.contains("MathJax")); // 默认使用 MathJax
    }
    
    #[test]
    fn test_latex_export() {
        let exporter = NotebookExporter::new();
        let mut notebook = Notebook::with_title("LaTeX 测试".to_string());
        notebook.add_cell(crate::NotebookCell::new_code("y = x^2".to_string()));
        
        let latex = exporter.export_to_latex(&notebook).unwrap();
        assert!(latex.contains("\\documentclass{article}"));
        assert!(latex.contains("LaTeX 测试"));
        assert!(latex.contains("y = x^2"));
        assert!(latex.contains("\\begin{lstlisting}"));
    }
    
    #[test]
    fn test_file_export() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.html");
        
        let exporter = NotebookExporter::new();
        let notebook = Notebook::with_title("文件导出测试".to_string());
        
        exporter.export_to_file(&notebook, &file_path, ExportFormat::Html).unwrap();
        assert!(file_path.exists());
        
        let content = std::fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("文件导出测试"));
    }
    
    #[test]
    fn test_escape_functions() {
        let exporter = NotebookExporter::new();
        
        // 测试 HTML 转义
        let html_escaped = exporter.escape_html("<script>alert('test')</script>");
        assert_eq!(html_escaped, "&lt;script&gt;alert(&#39;test&#39;)&lt;/script&gt;");
        
        // 测试 LaTeX 转义
        let latex_escaped = exporter.escape_latex("$x^2$ & {y}");
        assert!(latex_escaped.contains("\\$"));
        assert!(latex_escaped.contains("\\&"));
        assert!(latex_escaped.contains("\\{"));
    }
    
    #[test]
    fn test_custom_options() {
        let mut options = ExportOptions::default();
        options.include_outputs = false;
        options.math_renderer = MathRenderer::KaTeX;
        options.custom_css = Some("body { color: red; }".to_string());
        
        let exporter = NotebookExporter::with_options(options);
        let notebook = Notebook::with_title("自定义选项测试".to_string());
        
        let html = exporter.export_to_html(&notebook).unwrap();
        assert!(html.contains("KaTeX"));
        assert!(html.contains("color: red"));
    }
}