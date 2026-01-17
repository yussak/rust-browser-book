use core::cell::RefCell;

use alloc::rc::Rc;

use crate::renderer::{
    css::cssom::StyleSheet,
    dom::{
        api::get_target_element_node,
        node::{ElementKind, Node},
    },
    layout::layout_object::{create_layout_object, LayoutObject},
};

fn build_layout_tree(
    node: &Option<Rc<RefCell<Node>>>,
    parent_obj: &Option<Rc<RefCell<LayoutObject>>>,
    cssom: &StyleSheet,
) -> Option<Rc<RefCell<LayoutObject>>> {
    let mut target_node = node.clone();
    // ノードとなるLayoutObjectの作成を試みる
    // CSSでdisplay:noneが指定されている場合ノードは作成されない
    let mut layout_object = create_layout_object(node, parent_obj, cssom);
    // ノードが作成されなかった場合、DOMノードの兄弟ノードを使ってLayoutObjectの作成を試みる
    // LayoutObjectが作成されるまで兄弟ノードをたどり続ける
    while layout_object.is_none() {
        // if letという書き方がある
        // target_nodeに値がある場合、その値をnとする
        if let Some(n) = target_node {
            // 現在のノードの次の兄弟ノードに対して作成を試みる
            target_node = n.borrow().next_sibling().clone();
            layout_object = create_layout_object(&target_node, parent_obj, cssom);
        } else {
            // もし兄弟ノードがない場合、処理するべきDOMツリーは終了したので今まで作成したレイアウトツリーを返す
            return layout_object;
        }
    }

    // この時点で必ずlayout_objectの値はある状態

    // このtarget_nodeはlayout_objectが作成できたnode
    if let Some(n) = target_node {
        let original_first_child = n.borrow().first_child();
        let original_next_sibling = n.borrow().next_sibling();
        // 子、兄弟それぞれのレイアウトツリーを作成
        let mut first_child = build_layout_tree(&original_first_child, &layout_object, cssom);
        let mut next_sibling = build_layout_tree(&original_next_sibling, &None, cssom);

        if first_child.is_none() && original_first_child.is_some() {
            // 子ノードの兄弟ノードに対してレイアウトツリーの作成を試みる
            let mut original_dom_node = original_first_child
                .expect("first child should exist")
                .borrow()
                .next_sibling();

            loop {
                first_child = build_layout_tree(&original_dom_node, &layout_object, cssom);

                // 兄弟の兄弟ノード
                if first_child.is_none() && original_dom_node.is_some() {
                    original_dom_node = original_dom_node
                        .expect("next sibling should exist")
                        .borrow()
                        .next_sibling();

                    continue;
                }

                // first_childが作成できたか、子ノードの兄弟が無くなったらループを抜ける

                break;
            }
        }

        if next_sibling.is_none() && n.borrow().next_sibling().is_some() {
            let mut original_dom_node = original_next_sibling
                .expect("first child should exist")
                .borrow()
                .next_sibling();

            loop {
                next_sibling = build_layout_tree(&original_dom_node, &None, cssom);

                if next_sibling.is_none() && original_dom_node.is_some() {
                    original_dom_node = original_dom_node
                        .expect("next sibling should exist")
                        .borrow()
                        .next_sibling();

                    continue;
                }

                break;
            }
        }

        let obj = match layout_object {
            Some(ref obj) => obj,
            None => panic!("render object should exist here"),
        };

        // 作成した子と兄弟のレイアウトオブジェクトを現在参照しているレイアウトオブジェクトの子、兄弟ノードとして追加
        obj.borrow_mut().set_first_child(first_child);
        obj.borrow_mut().set_next_sibling(next_sibling);
    }

    layout_object
}

#[derive(Debug, Clone)]
pub struct LayoutView {
    root: Option<Rc<RefCell<LayoutObject>>>,
}

impl LayoutView {
    pub fn new(root: Rc<RefCell<Node>>, cssom: &StyleSheet) -> Self {
        // レイアウトツリーは描画される要素だけを持つツリーなので、
        // <body>タグを取得し、その子要素以下をレイアウトツリーのノードに変換する
        let body_root = get_target_element_node(Some(root), ElementKind::Body);

        let mut tree = Self {
            root: build_layout_tree(&body_root, &None, cssom),
        };

        tree.update_layout();

        tree
    }

    pub fn root(&self) -> Option<Rc<RefCell<LayoutObject>>> {
        self.root.clone()
    }
}
