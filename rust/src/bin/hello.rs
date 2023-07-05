#![feature(type_alias_impl_trait)]
#![feature(trait_alias)]
#![feature(generators)]

use futures::Stream;
pub type V = usize;

mod merge_sort {

    use std::{
        collections::{binary_heap::PeekMut, BinaryHeap},
        error::Error,
    };

    use futures::{Stream, StreamExt};
    use futures_async_stream::{stream, try_stream};

    use super::*;

    pub trait MergeSortKey = Eq + PartialEq + Ord + PartialOrd;

    struct Node<K: MergeSortKey, S> {
        stream: S,

        /// The next item polled from `stream` previously. Since the `eq` and `cmp` must be synchronous
        /// functions, we need to implement peeking manually.
        peeked: (K, V),
    }

    impl<K: MergeSortKey, S> PartialEq for Node<K, S> {
        fn eq(&self, other: &Self) -> bool {
            match self.peeked.0 == other.peeked.0 {
                true => unreachable!("primary key from different iters should be unique"),
                false => false,
            }
        }
    }
    impl<K: MergeSortKey, S> Eq for Node<K, S> {}

    impl<K: MergeSortKey, S> PartialOrd for Node<K, S> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<K: MergeSortKey, S> Ord for Node<K, S> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            // The heap is a max heap, so we need to reverse the order.
            self.peeked.0.cmp(&other.peeked.0).reverse()
        }
    }

    #[try_stream(ok=(K, V), error=E)]
    // #[stream(item = (K, V))]
    pub async fn merge_sort<'a, K, E, R>(streams: Vec<R>)
    where
        K: MergeSortKey + 'a,
        E: 'a,
        R: Stream<Item = Result<(K, V), E>> + 'a + Unpin,
    {
        let mut heap = BinaryHeap::new();
        for mut stream in streams {
            if let Some(peeked) = stream.next().await.transpose()? {
                heap.push(Node { stream, peeked });
            }
        }
        while let Some(mut node) = heap.peek_mut() {
            // Note: If the `next` returns `Err`, we'll fail to yield the previous item.
            yield match node.stream.next().await.transpose()? {
                // There still remains data in the stream, take and update the peeked value.
                Some(new_peeked) => std::mem::replace(&mut node.peeked, new_peeked),
                // This stream is exhausted, remove it from the heap.
                None => PeekMut::pop(node).peeked,
            };
        }
    }
}

mod abc {
    use super::*;

    type Op = impl Stream<Item = Result<(usize, V), String>>;

    async fn a() -> Op {
        futures::stream::iter(vec![])
    }

    async fn b() -> Op {
        // merge_sort::merge_sort(vec![futures::stream::empty()])
        merge_sort::merge_sort(vec![a().await])
    }
}

#[tokio::main]
async fn main() {
    println!("Hello async world!");
}
