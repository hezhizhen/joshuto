extern crate fs_extra;
extern crate ncurses;

use std;

use std::fmt;

use joshuto;
use joshuto::command;
use joshuto::preview;
use joshuto::ui;

#[derive(Clone, Debug)]
pub struct ParentDirectory;

impl ParentDirectory {
    pub fn new() -> Self { ParentDirectory }
    pub fn command() -> &'static str { "parent_directory" }

    pub fn parent_directory(context: &mut joshuto::JoshutoContext) -> bool
    {
        let curr_tab = &mut context.tabs[context.tab_index];
        if curr_tab.curr_path.pop() == false {
            return false;
        }

        match std::env::set_current_dir(&curr_tab.curr_path) {
            Ok(_) => {
                {
                    let curr_list = curr_tab.curr_list.take();
                    curr_tab.history.put_back(curr_list);

                    let parent_list = curr_tab.parent_list.take();
                    curr_tab.curr_list = parent_list;
                }

                match curr_tab.curr_path.parent() {
                    Some(parent) => {
                        curr_tab.parent_list = match curr_tab.history.pop_or_create(&parent, &context.config_t.sort_type) {
                            Ok(s) => Some(s),
                            Err(e) => {
                                ui::wprint_err(&context.views.left_win, e.to_string().as_str());
                                None
                            },
                        };
                    },
                    None => {
                        ncurses::werase(context.views.left_win.win);
                        ncurses::wnoutrefresh(context.views.left_win.win);
                    },
                }

                ui::redraw_view(&context.config_t, &context.theme_t,
                        &context.views.left_win, curr_tab.parent_list.as_mut());
                ui::redraw_view_detailed(&context.config_t, &context.theme_t,
                        &context.views.mid_win, curr_tab.curr_list.as_mut());

                ui::redraw_status(&context.theme_t, &context.views,
                        curr_tab.curr_list.as_ref(),
                        &curr_tab.curr_path,
                        &context.username, &context.hostname);
                return true;
            },
            Err(e) => {
                ui::wprint_err(&context.views.bot_win, e.to_string().as_str());
                return false;
            },
        };

    }
}

impl command::JoshutoCommand for ParentDirectory {}

impl std::fmt::Display for ParentDirectory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        f.write_str(Self::command())
    }
}

impl command::Runnable for ParentDirectory {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        if Self::parent_directory(context) {
            preview::preview_file(context);
            ncurses::doupdate();
        }
    }
}
