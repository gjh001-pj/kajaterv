

// #[macro_export]
// macro_rules! tr {
//     ($re_draw_msg:expr, $dragover_index:expr, $dragged_index:expr, $link:expr, $index:expr, $data_vec:expr, {$($inner:tt)*}) => {
//         // let ondragstart = move |e: DragEvent| {
//         //     e.data_transfer().unwrap().set_drag_image(&web_sys::HtmlImageElement::new().unwrap(), 0, 0);
//         //     $dragged_index = Some($index);
//         //     $link.send_message($re_draw_msg);
//         // };

//         // let ondragenter = move |_| {
//         //     $dragover_index = Some($index);
//         //     $link.send_message($re_draw_msg);
//         // };

//         // let ondragover=Callback::from(move |e: DragEvent| e.prevent_default())

//         // let ondrop = move |_| {
//         //     let src_index = $dragged_index.unwrap();
//         //     if src_index != $index {
//         //         // $data_vec.swap(src_index, $index);
//         //         let item = $data_vec.remove(src_index);
//         //         $data_vec.insert($index, item);

//         //         $link.send_message($re_draw_msg);
//         //     };
//         //     $dragged_index = None;
//         //     $dragover_index = None;
//         // };

//         // let row_class = if $dragged_index == Some(index) {"dragged-row"}
//         // else if $dragover_index == Some(index) {"drag-over-row"} 
//         // else {""};

//         html!{
//             <tr
//                 key={$index}
//                 draggable={"true"}
//                 class={if $dragged_index == Some($index) {"dragged-row"}
//                     else if $dragover_index == Some($index) {"drag-over-row"} 
//                     else {""}}

//                 ondragstart= {move |e: DragEvent| {
//                         e.data_transfer().unwrap().set_drag_image(&web_sys::HtmlImageElement::new().unwrap(), 0, 0);
//                         $dragged_index = Some($index);
//                         $link.send_message($re_draw_msg);
//                 }}
//                 ondragenter = {move |_| {
//                     $dragover_index = Some($index);
//                     $link.send_message($re_draw_msg);
//                 }}
//                 ondragover = {Callback::from(move |e: DragEvent| e.prevent_default())}
//                 ondrop = {move |_| {
//                     let src_index = $dragged_index.unwrap();
//                     if src_index != $index {
//                         // $data_vec.swap(src_index, $index);
//                         let item = $data_vec.remove(src_index);
//                         $data_vec.insert($index, item);

//                         $link.send_message($re_draw_msg);
//                     };
//                     $dragged_index = None;
//                     $dragover_index = None;
//                 }}
//             >
//                 $($inner)*
//             </tr>
//         }
//     };
// }