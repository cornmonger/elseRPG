use std::collections::HashMap;

trait NodeTrait {
    fn name() -> &str;
    fn children() -> &[Node];
}

pub enum Node {
    Entity (EntityNode),
    ComponentRelation(ComponentRelationNode),
    AttachmentRelation(AttachmentNode),
}

impl NodeTrait for Node {

}

pub struct EntityNode {
    name: &'static str,
    children: [Node]
}

pub struct ComponentRelationNode {
    name: &'static str,
    child_entity: EntityNode
}

pub struct AttachmentRelationNode {
    name: &'static str,
}

pub struct Tree {
    entities: [EntityNode],
} 

const tree: Tree = Tree {
    entities: [EntityNode {
        name: "humanoid",
        children: [
            ComponentRelationNode {
                name: "head",
                child_entity: EntityNode {
                    name: "head",
                    children: [
                        AttachmentRelationNode {
                            name: "WearOn"
                        }
                    ]
                }  
            },
            ComponentRelationNode {
                name: "torso",
                child_entity: EntityNode {
                    name: "torso",
                    children: [
                        ComponentRelationNode: {
                            name: "chest",
                            child_entity: EntityNode {
                                name: "chest",
                                children: [

                                ]
                            }
                        }
                    ]}
                }
            }
        ]
    }]
};

