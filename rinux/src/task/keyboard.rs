use crate::{print, print_err, print_ok};
use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::{
    stream::{Stream, StreamExt},
    task::AtomicWaker,
};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            print_err!("[ERR] scancode queue full; dropping keyboard input\n");
        } else {
            WAKER.wake();
        }
    } else {
        print_err!("[ERR] scancode queue uninitialized\n");
    }
}

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        match SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100)) {
            Ok(_) => {
                if crate::conf::QUIET_BOOT != true {
                    print_ok!("[OK] Scancode initialized\n");
                };
            },
            Err(_) => {
                print_err!("[ERR] ScancodeStream::new should only be called once");
                panic!("ScancodeStream::new should only be called once");
            }
        }
        ScancodeStream {
            _private: ()
        }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE.try_get()
        .expect("scancode queue not initialized");

        if let Ok(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Ok(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}

pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        print!("{}", character);
                    },
                    DecodedKey::RawKey(key) => print!("{:?}", key),
                }
            }
        }
    }
}

pub async fn init() {
    let _ = ScancodeStream::new();
    let _ = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);
}