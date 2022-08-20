use dialoguer::console::{style, Emoji};
use dialoguer::theme::{ColorfulTheme, Theme as ThemeTrait};

#[derive(Default)]
pub struct Theme {
    inner: ColorfulTheme,
}

impl Theme {
    pub fn book() -> Theme {
        Theme::default().prompt_prefix("📚")
    }

    pub fn chapter() -> Theme {
        Theme::default().prompt_prefix("📖")
    }

    pub fn verse() -> Theme {
        Theme::default().prompt_prefix("📜")
    }

    fn prompt_prefix(mut self, val: &str) -> Theme {
        self.inner.prompt_prefix = style(Emoji::new(val, "?").to_string());
        self
    }
}

impl ThemeTrait for Theme {
    fn format_prompt(&self, f: &mut dyn std::fmt::Write, prompt: &str) -> std::fmt::Result {
        self.inner.format_prompt(f, prompt)
    }

    fn format_error(&self, f: &mut dyn std::fmt::Write, err: &str) -> std::fmt::Result {
        self.inner.format_error(f, err)
    }

    fn format_confirm_prompt(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        default: Option<bool>,
    ) -> std::fmt::Result {
        self.inner.format_confirm_prompt(f, prompt, default)
    }

    fn format_confirm_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        selection: Option<bool>,
    ) -> std::fmt::Result {
        self.inner
            .format_confirm_prompt_selection(f, prompt, selection)
    }

    fn format_input_prompt(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        default: Option<&str>,
    ) -> std::fmt::Result {
        self.inner.format_input_prompt(f, prompt, default)
    }

    fn format_input_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> std::fmt::Result {
        self.inner.format_input_prompt_selection(f, prompt, sel)
    }

    fn format_password_prompt(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
    ) -> std::fmt::Result {
        self.format_input_prompt(f, prompt, None)
    }

    fn format_password_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
    ) -> std::fmt::Result {
        self.format_input_prompt_selection(f, prompt, "[hidden]")
    }

    fn format_select_prompt(&self, f: &mut dyn std::fmt::Write, prompt: &str) -> std::fmt::Result {
        self.format_prompt(f, prompt)
    }

    fn format_select_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> std::fmt::Result {
        self.format_input_prompt_selection(f, prompt, sel)
    }

    fn format_multi_select_prompt(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
    ) -> std::fmt::Result {
        self.format_prompt(f, prompt)
    }

    fn format_sort_prompt(&self, f: &mut dyn std::fmt::Write, prompt: &str) -> std::fmt::Result {
        self.format_prompt(f, prompt)
    }

    fn format_multi_select_prompt_selection(
        &self,
        _f: &mut dyn std::fmt::Write,
        _prompt: &str,
        _selections: &[&str],
    ) -> std::fmt::Result {
        Ok(())
    }

    fn format_sort_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> std::fmt::Result {
        self.format_multi_select_prompt_selection(f, prompt, selections)
    }

    fn format_select_prompt_item(
        &self,
        f: &mut dyn std::fmt::Write,
        text: &str,
        active: bool,
    ) -> std::fmt::Result {
        self.inner.format_select_prompt_item(f, text, active)
    }

    fn format_multi_select_prompt_item(
        &self,
        f: &mut dyn std::fmt::Write,
        text: &str,
        checked: bool,
        active: bool,
    ) -> std::fmt::Result {
        self.inner
            .format_multi_select_prompt_item(f, text, checked, active)
    }

    fn format_sort_prompt_item(
        &self,
        f: &mut dyn std::fmt::Write,
        text: &str,
        picked: bool,
        active: bool,
    ) -> std::fmt::Result {
        self.inner.format_sort_prompt_item(f, text, picked, active)
    }

    fn format_fuzzy_select_prompt(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        search_term: &[String],
        cursor_pos: usize,
    ) -> std::fmt::Result {
        self.inner
            .format_fuzzy_select_prompt(f, prompt, search_term, cursor_pos)
    }
}
