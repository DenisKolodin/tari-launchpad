// Copyright 2023. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

use std::borrow::Cow;

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Row, Table},
};

use crate::{
    component::{elements::block_with_title, Component, ComponentEvent, Frame, Input, Pass},
    focus_id,
    state::{
        focus::{self, Focus},
        AppState,
    },
};

pub static BALANCE: Focus = focus_id!();

pub struct BalanceWidget {}

impl BalanceWidget {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for BalanceWidget {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if state.focus_on == BALANCE {
            // TODO: Set focus to the particular value
            // self.password.set_focus(true);
            match event.pass() {
                Pass::Up | Pass::Leave => {
                    state.focus_on(focus::ROOT);
                },
                Pass::Enter | Pass::Space => {
                    // TODO: Toggle the base node state
                },
                _ => {},
            }
        }
    }
}

impl<B: Backend> Component<B> for BalanceWidget {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Balance"), state.focus_on == BALANCE);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);

        let constraints = [
            Constraint::Length(1),
            Constraint::Length(4),
            // Constraint::Percentage(50),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(1),
        ];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_rect);

        let mut available = None;
        let mut incoming = None;
        let mut outgoing = None;
        let mut timelocked = None;
        if let Some(balance) = state.state.wallet.balance.as_ref() {
            available = Some(balance.available);
            incoming = Some(balance.pending_incoming);
            outgoing = Some(balance.pending_outgoing);
            timelocked = Some(balance.timelocked);
        }
        let rows = rows([
            ("Available", available),
            ("Incoming", incoming),
            ("Outgoing", outgoing),
            ("Timelocked", timelocked),
        ]);
        let table = Table::new(rows)
            .widths(&[Constraint::Percentage(40), Constraint::Percentage(60)])
            .column_spacing(2);
        f.render_widget(table, v_chunks[1]);
    }
}

fn rows<'a>(items: impl IntoIterator<Item = (&'a str, Option<u64>)>) -> Vec<Row<'a>> {
    let mut rows = Vec::new();
    for (title, value) in items {
        let value = value
            .map(|v| Cow::Owned(v.to_string()))
            .unwrap_or_else(|| Cow::Borrowed("-"));
        let items = vec![Cow::Borrowed(title), value];
        let row = Row::new(items);
        rows.push(row);
    }
    rows
}
