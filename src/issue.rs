// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{Span, Spanned};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Warning,
    Error,
}

pub struct Issue {
    pub level: Level,
    pub message: String,
    pub span: Span,
    pub fragments: Vec<(String, Span)>,
}

impl Issue {
    pub fn err(message: impl Into<String>, span: &impl Spanned) -> Self {
        Issue {
            level: Level::Error,
            message: message.into(),
            span: span.span(),
            fragments: Vec::new(),
        }
    }
    pub fn warn(message: impl Into<String>, span: &impl Spanned) -> Self {
        Issue {
            level: Level::Warning,
            message: message.into(),
            span: span.span(),
            fragments: Vec::new(),
        }
    }
    pub fn frag(mut self, message: impl Into<String>, span: &impl Spanned) -> Self {
        self.fragments.push((message.into(), span.span()));
        self
    }
    pub fn todo(span: &impl Spanned) -> Self {
        Issue::err("Not implemented", span)
    }
}
