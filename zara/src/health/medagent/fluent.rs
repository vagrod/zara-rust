use crate::health::medagent::{CurveType, MedicalAgent, MedicalAgentGroup};
use crate::health::MedicalAgentBuilder;

impl MedicalAgentBuilder {
    fn as_agent_curve(&self) -> &dyn AgentCurve { self }
    fn as_agent_duration(&self) -> &dyn AgentDuration { self }
    fn as_agent_items(&self) -> &dyn AgentItems { self }
    fn as_agent_end(&self) -> &dyn AgentEnd { self }
}

pub trait AgentStart {
    /// Unique name of a medical agent. Will become its key
    fn for_agent(&self, name: &str) -> &dyn AgentCurve;
}

pub trait AgentCurve {
    /// Activation curve type for this agent
    fn activates(&self, curve_type: CurveType) -> &dyn AgentDuration;
}

pub trait AgentDuration {
    /// Duration, in game minutes, of a single agent dose to have an effect
    fn and_lasts_for_minutes(&self, game_minutes: f32) -> &dyn AgentItems;
}

pub trait AgentItems {
    /// What kinds of inventory items are describing this agent
    ///
    /// # Examples
    /// ```
    /// includes(
    ///     vec![
    ///         "Syringe with Epinephrine",
    ///         "Epinephrine Pills",
    ///         "Yka-Yka Leaf"
    ///     ]
    /// )
    /// ```
    fn includes(&self, items: Vec<&str>) -> &dyn AgentEnd;
}

pub trait AgentEnd {
    /// Builds resulted medical agent according with the information provided
    fn build(&self) -> MedicalAgent;
}

impl AgentStart for MedicalAgentBuilder {
    fn for_agent(&self, name: &str) -> &dyn AgentCurve {
        self.name.replace(name.to_string());

        self.as_agent_curve()
    }
}
impl AgentCurve for MedicalAgentBuilder {
    fn activates(&self, curve_type: CurveType) -> &dyn AgentDuration {
        self.curve_type.replace(curve_type);

        self.as_agent_duration()
    }
}
impl AgentDuration for MedicalAgentBuilder {
    fn and_lasts_for_minutes(&self, game_minutes: f32) -> &dyn AgentItems {
        self.duration_minutes.set(game_minutes);

        self.as_agent_items()
    }
}
impl AgentItems for MedicalAgentBuilder {
    fn includes(&self, items: Vec<&str>) -> &dyn AgentEnd {
        self.items.replace(items.iter().map(|x| x.to_string()).collect());

        self.as_agent_end()
    }
}
impl AgentEnd for MedicalAgentBuilder {
    fn build(&self) -> MedicalAgent {
        MedicalAgent::new(
            self.name.borrow().to_string(),
            *self.curve_type.borrow(),
            self.duration_minutes.get(),
            MedicalAgentGroup::new(
                self.items.borrow().iter().map(|x| x.to_string()).collect()
            ))
    }
}