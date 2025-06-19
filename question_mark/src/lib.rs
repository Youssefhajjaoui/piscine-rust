pub struct One {
    pub first_layer: Option<Two>,
}
pub struct Two {
    pub second_layer: Option<Three>,
}
pub struct Three {
    pub third_layer: Option<Four>,
}
pub struct Four {
    pub fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        let two = self.first_layer.unwrap();
        let three = (two.second_layer).unwrap();
        let Four = (three.third_layer).unwrap();
        return Four.fourth_layer;
    }
}
