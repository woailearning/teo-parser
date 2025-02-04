use crate::ast::action::ActionGroupDeclaration;
use crate::ast::config::Config;
use crate::ast::config_declaration::ConfigDeclaration;
use crate::ast::constant::Constant;
use crate::ast::data_set::DataSet;
use crate::ast::decorator_declaration::DecoratorDeclaration;
use crate::ast::import::Import;
use crate::ast::interface::InterfaceDeclaration;
use crate::ast::middleware::Middleware;
use crate::ast::model::Model;
use crate::ast::namespace::Namespace;
use crate::ast::pipeline_item_declaration::PipelineItemDeclaration;
use crate::ast::r#enum::Enum;
use crate::ast::span::Span;
use crate::ast::struct_declaration::StructDeclaration;

#[derive(Debug)]
pub(crate) enum Top {
    Import(Import),
    Config(Config),
    ConfigDeclaration(ConfigDeclaration),
    Constant(Constant),
    Enum(Enum),
    Model(Model),
    DataSet(DataSet),
    Middleware(Middleware),
    ActionGroup(ActionGroupDeclaration),
    Interface(InterfaceDeclaration),
    Namespace(Namespace),
    DecoratorDeclaration(DecoratorDeclaration),
    PipelineItemDeclaration(PipelineItemDeclaration),
    StructDeclaration(StructDeclaration),
}

impl Top {

    pub(crate) fn id(&self) -> usize {
        match self {
            Top::Import(i) => i.id(),
            Top::Constant(c) => c.id(),
            Top::Enum(e) => e.id(),
            Top::Model(m) => m.id(),
            Top::Config(c) => c.id(),
            Top::ConfigDeclaration(c) => c.id(),
            Top::DataSet(d) => d.id(),
            Top::Middleware(m) => m.id(),
            Top::ActionGroup(a) => a.id(),
            Top::Interface(i) => i.id(),
            Top::Namespace(n) => n.id(),
            Top::DecoratorDeclaration(d) => d.id(),
            Top::PipelineItemDeclaration(p) => p.id(),
            Top::StructDeclaration(s) => s.id(),
        }
    }

    pub(crate) fn name(&self) -> Option<&str> {
        match self {
            Top::Import(i) => None,
            Top::Constant(c) => Some(c.identifier.name()),
            Top::Enum(e) => Some(e.identifier.name()),
            Top::Model(m) => Some(m.identifier.name()),
            Top::Config(c) => Some(c.name()),
            Top::ConfigDeclaration(c) => Some(c.identifier.name()),
            Top::DataSet(d) => Some(d.identifier.name()),
            Top::Middleware(m) => Some(m.identifier.name()),
            Top::ActionGroup(a) => Some(a.identifier.name()),
            Top::Interface(i) => Some(i.identifier.name()),
            Top::Namespace(n) => Some(n.identifier.name()),
            Top::DecoratorDeclaration(d) => Some(d.identifier.name()),
            Top::PipelineItemDeclaration(p) => Some(p.identifier.name()),
            Top::StructDeclaration(s) => Some(s.identifier.name()),
        }
    }

    pub(crate) fn path(&self) -> &Vec<usize> {
        match self {
            Top::Import(i) => &i.path,
            Top::Constant(c) => &c.path,
            Top::Enum(e) => &e.path,
            Top::Model(m) => &m.path,
            Top::Config(c) => &c.path,
            Top::ConfigDeclaration(c) => &c.path,
            Top::DataSet(d) => &d.path,
            Top::Middleware(m) => &m.path,
            Top::ActionGroup(a) => &a.path,
            Top::Interface(i) => &i.path,
            Top::Namespace(n) => &n.path,
            Top::DecoratorDeclaration(d) => &d.path,
            Top::PipelineItemDeclaration(p) => &p.path,
            Top::StructDeclaration(s) => &s.path,
        }
    }

    pub(crate) fn span(&self) -> Span {
        match self {
            Top::Import(i) => i.span,
            Top::Constant(c) => c.span,
            Top::Enum(e) => e.span,
            Top::Model(m) => m.span,
            Top::Config(c) => c.span,
            Top::ConfigDeclaration(c) => c.span,
            Top::DataSet(d) => d.span,
            Top::Middleware(m) => m.span,
            Top::ActionGroup(a) => a.span,
            Top::Interface(i) => i.span,
            Top::Namespace(n) => n.span,
            Top::DecoratorDeclaration(d) => d.span,
            Top::PipelineItemDeclaration(p) => p.span,
            Top::StructDeclaration(s) => s.span,
        }
    }

    pub(crate) fn as_import(&self) -> Option<&Import> {
        match self {
            Top::Import(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn is_import(&self) -> bool {
        self.as_import().is_some()
    }

    pub(crate) fn as_constant(&self) -> Option<&Constant> {
        match self {
            Top::Constant(c) => Some(c),
            _ => None,
        }
    }

    pub(crate) fn is_constant(&self) -> bool {
        self.as_constant().is_some()
    }

    pub(crate) fn as_enum(&self) -> Option<&Enum> {
        match self {
            Top::Enum(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn is_enum(&self) -> bool {
        self.as_enum().is_some()
    }

    pub(crate) fn as_model(&self) -> Option<&Model> {
        match self {
            Top::Model(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn is_model(&self) -> bool {
        self.as_model().is_some()
    }

    pub(crate) fn as_config(&self) -> Option<&Config> {
        match self {
            Top::Config(c) => Some(c),
            _ => None
        }
    }

    pub(crate) fn is_config(&self) -> bool {
        self.as_config().is_some()
    }

    pub(crate) fn as_config_declaration(&self) -> Option<&ConfigDeclaration> {
        match self {
            Top::ConfigDeclaration(c) => Some(c),
            _ => None
        }
    }

    pub(crate) fn is_config_declaration(&self) -> bool {
        self.as_config_declaration().is_some()
    }


    pub(crate) fn as_data_set(&self) -> Option<&DataSet> {
        match self {
            Top::DataSet(d) => Some(d),
            _ => None,
        }
    }

    pub(crate) fn is_data_set(&self) -> bool {
        self.as_data_set().is_some()
    }

    pub(crate) fn as_middleware(&self) -> Option<&Middleware> {
        match self {
            Top::Middleware(m) => Some(m),
            _ => None,
        }
    }

    pub(crate) fn is_middleware(&self) -> bool {
        self.as_middleware().is_some()
    }

    pub(crate) fn as_action_group(&self) -> Option<&ActionGroupDeclaration> {
        match self {
            Top::ActionGroup(m) => Some(m),
            _ => None,
        }
    }

    pub(crate) fn is_action_group(&self) -> bool {
        self.as_action_group().is_some()
    }

    pub(crate) fn as_interface(&self) -> Option<&InterfaceDeclaration> {
        match self {
            Top::Interface(m) => Some(m),
            _ => None,
        }
    }

    pub(crate) fn is_interface(&self) -> bool {
        self.as_interface().is_some()
    }

    pub(crate) fn as_namespace(&self) -> Option<&Namespace> {
        match self {
            Top::Namespace(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn is_namespace(&self) -> bool {
        self.as_namespace().is_some()
    }

    pub(crate) fn as_decorator_declaration(&self) -> Option<&DecoratorDeclaration> {
        match self {
            Top::DecoratorDeclaration(d) => Some(d),
            _ => None,
        }
    }

    pub(crate) fn is_decorator_declaration(&self) -> bool {
        self.as_decorator_declaration().is_some()
    }

    pub(crate) fn as_pipeline_item_declaration(&self) -> Option<&PipelineItemDeclaration> {
        match self {
            Top::PipelineItemDeclaration(p) => Some(p),
            _ => None,
        }
    }

    pub(crate) fn is_pipeline_item_declaration(&self) -> bool {
        self.as_pipeline_item_declaration().is_some()
    }

    pub(crate) fn as_struct_declaration(&self) -> Option<&StructDeclaration> {
        match self {
            Top::StructDeclaration(s) => Some(s),
            _ => None,
        }
    }

    pub(crate) fn is_struct_declaration(&self) -> bool {
        self.as_struct_declaration().is_some()
    }
}
