use std::collections::HashMap;

trait NodeTrait {
    fn name() -> &'static str;
    fn children() -> &'static [Node];
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
    root_node: EntityNode,
} 

pub struct Prototype {
    tree: Tree
}


const humanoid: Prototype = Prototype {
    tree: Tree {
        root_node: EntityNode {  // humanoid
            name: "humanoid", 
            children: [
                ComponentRelationNode {   
                    name: "head",  
                    child_entity: EntityNode {  // humanoid::head
                        name: "head",
                        children: [
                            AttachmentRelationNode {  
                                name: "WearOn"
                            }
                        ]
                    }
                }
            ]
        }
    }
};

