use crate::conversion::FromGValue;
use crate::process::traversal::step::by::IntoByStep;
use crate::process::traversal::step::has::IntoHasStep;
use crate::process::traversal::step::select::IntoSelectStep;
use crate::process::traversal::strategies::TraversalStrategies;
use crate::process::traversal::Bytecode;
use crate::structure::Either2;
use crate::structure::Labels;
use crate::{structure::GProperty, Edge, GValue, GremlinResult, List, Map, Path, Vertex};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct GraphTraversal<S, E: FromGValue> {
    start: PhantomData<S>,
    end: PhantomData<E>,
    strategies: TraversalStrategies,
    pub(crate) bytecode: Bytecode,
}

impl<S, E: FromGValue> GraphTraversal<S, E> {
    pub fn new(strategies: TraversalStrategies, bytecode: Bytecode) -> GraphTraversal<S, E> {
        GraphTraversal {
            start: PhantomData,
            end: PhantomData,
            bytecode,
            strategies,
        }
    }
    pub fn bytecode(&self) -> &Bytecode {
        &self.bytecode
    }

    pub fn has_label<L>(mut self, labels: L) -> Self
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("hasLabel"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );
        self
    }

    pub fn add_v<T>(mut self, label: T) -> GraphTraversal<Vertex, Vertex>
    where
        T: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("addV"),
            label.into().0.into_iter().map(GValue::from).collect(),
        );

        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn property<T>(mut self, key: &str, value: T) -> Self
    where
        T: Into<GValue>,
    {
        self.bytecode.add_step(
            String::from("property"),
            vec![String::from(key).into(), value.into()],
        );
        self
    }

    pub fn has<A>(mut self, step: A) -> Self
    where
        A: IntoHasStep,
    {
        self.bytecode
            .add_step(String::from("has"), step.into_step().take_params());
        self
    }

    pub fn has_not<A>(mut self, key: A) -> Self
    where
        A: Into<String>,
    {
        self.bytecode
            .add_step(String::from("hasNot"), vec![key.into().into()]);
        self
    }
    pub fn as_<T>(mut self, alias: T) -> GraphTraversal<S, E>
    where
        T: Into<String>,
    {
        self.bytecode
            .add_step(String::from("as"), vec![alias.into().into()]);

        self
    }

    pub fn add_e<T>(mut self, label: T) -> GraphTraversal<S, Edge>
    where
        T: Into<String>,
    {
        self.bytecode
            .add_step(String::from("addE"), vec![label.into().into()]);

        GraphTraversal::new(self.strategies, self.bytecode)
    }
    pub fn out<L>(mut self, labels: L) -> GraphTraversal<S, Vertex>
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("out"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn out_e<L>(mut self, labels: L) -> GraphTraversal<S, Edge>
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("outE"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn out_v(mut self) -> GraphTraversal<S, Vertex> {
        self.bytecode.add_step(String::from("outV"), vec![]);

        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn in_<L>(mut self, labels: L) -> GraphTraversal<S, Vertex>
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("in"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn in_e<L>(mut self, labels: L) -> GraphTraversal<S, Edge>
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("inE"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn in_v(mut self) -> GraphTraversal<S, Vertex> {
        self.bytecode.add_step(String::from("inV"), vec![]);

        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn label(mut self) -> GraphTraversal<S, String> {
        self.bytecode.add_step(String::from("label"), vec![]);

        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn to_list(&self) -> GremlinResult<Vec<E>> {
        self.strategies.apply(self)?.collect()
    }

    pub fn from<A>(mut self, target: A) -> GraphTraversal<S, E>
    where
        A: Into<Either2<String, Vertex>>,
    {
        self.bytecode
            .add_step(String::from("from"), vec![target.into().into()]);

        self
    }

    pub fn to<A>(mut self, target: A) -> GraphTraversal<S, E>
    where
        A: Into<Either2<String, Vertex>>,
    {
        self.bytecode
            .add_step(String::from("to"), vec![target.into().into()]);

        self
    }

    pub fn properties<L>(mut self, labels: L) -> GraphTraversal<S, GProperty>
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("properties"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );
        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn property_map<L>(mut self, labels: L) -> GraphTraversal<S, Map>
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("propertyMap"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );
        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn values<L>(mut self, labels: L) -> GraphTraversal<S, GValue>
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("values"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );
        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn count(mut self) -> GraphTraversal<S, i64> {
        self.bytecode.add_step(String::from("count"), vec![]);
        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn group_count(mut self) -> GraphTraversal<S, Map> {
        self.bytecode.add_step(String::from("groupCount"), vec![]);
        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn group(mut self) -> GraphTraversal<S, Map> {
        self.bytecode.add_step(String::from("group"), vec![]);
        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn by<A>(mut self, step: A) -> Self
    where
        A: IntoByStep,
    {
        self.bytecode
            .add_step(String::from("by"), step.into_step().take_params());
        self
    }

    pub fn select<A>(mut self, step: A) -> GraphTraversal<S, GValue>
    where
        A: IntoSelectStep,
    {
        self.bytecode
            .add_step(String::from("select"), step.into_step().take_params());
        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn fold(mut self) -> GraphTraversal<S, List> {
        self.bytecode.add_step(String::from("fold"), vec![]);
        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn path(mut self) -> GraphTraversal<S, Path> {
        self.bytecode.add_step(String::from("path"), vec![]);
        GraphTraversal::new(self.strategies, self.bytecode)
    }
}