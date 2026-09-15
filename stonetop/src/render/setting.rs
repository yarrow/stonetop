//! The walk of the `SettingOverviewFixed` into one Markdown document: H1 the title, H2 each
//! section, H3 each subsection, every body through the same converter and spoken forms as
//! the playbooks. The heading levels come from the nesting alone.

use crate::fixed::{SettingOverviewFixed, SettingSection};

use super::document::{Document, speak};

/// `overview` as one Markdown document, ending in a newline.
#[must_use]
pub fn render_setting_markdown(overview: &SettingOverviewFixed) -> String {
    let mut doc = Document::default();
    doc.block(format!("# {}", speak(overview.title)));
    for section in overview.sections {
        section_block(&mut doc, section);
    }
    doc.finish()
}

fn section_block(doc: &mut Document, section: &SettingSection) {
    doc.block(format!("## {}", speak(section.heading)));
    doc.block(speak(section.html));
    for subsection in section.subsections {
        doc.block(format!("### {}", speak(subsection.heading)));
        doc.block(speak(subsection.html));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixed::SettingSubsection;

    #[test]
    fn a_subsection_is_an_h3_under_its_sections_h2() {
        let overview = SettingOverviewFixed {
            title: "Setting Overview",
            sections: &[
                SettingSection {
                    heading: "The premise",
                    html: "<p>You play the heroes of <strong>Stonetop</strong>.</p>",
                    subsections: &[],
                },
                SettingSection {
                    heading: "Stonetop",
                    html: "<ul><li><strong>Size</strong> <em>village</em></li></ul>",
                    subsections: &[SettingSubsection {
                        heading: "Map: The village of Stonetop",
                        html: "<p>The Stone is in the middle.</p><p>Houses all around.</p>",
                    }],
                },
            ],
        };
        assert_eq!(
            render_setting_markdown(&overview),
            "# Setting Overview\n\n\
             ## The premise\n\n\
             You play the heroes of **Stonetop**.\n\n\
             ## Stonetop\n\n\
             - **Size** *village*\n\n\
             ### Map: The village of Stonetop\n\n\
             The Stone is in the middle.\n\n\
             Houses all around.\n"
        );
    }
}
