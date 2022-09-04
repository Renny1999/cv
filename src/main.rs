use opencv as cv2;

fn main() {
    let mut cam = cv2::videoio::
        VideoCapture::new(0,cv2::videoio::CAP_ANY).unwrap();

    // while true {
    //
    // }

    println!("Hello, world!");
}
