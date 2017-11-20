// Task: Range Minimum Query
use std::fmt;

// #[derive(Debug)]
struct Bucket<T> {
    n: usize,
    array: Vec<T>,
    buckets: Vec<T>
}

impl<T> Bucket<T> {
    fn sqrt(n: usize) -> usize {
        (n as f64).sqrt() as usize
    }
}

impl<T: fmt::Debug> fmt::Debug for Bucket<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\n\tn: {},\n\tarray: {:?},\n\tbuckets: {:?}\n}}", self.n, self.array, self.buckets)
    }
}

impl<T: PartialOrd+Clone+fmt::Debug> Bucket<T> {
    fn new(n: usize, init: &T) -> Bucket<T> {
        let sqrt_n = Bucket::<T>::sqrt(n);
        Bucket{
            n: n,
            array: vec![init.clone(); n],
            buckets: vec![init.clone();  sqrt_n + if n%sqrt_n == 0 {0} else {1}],
        }
    }

    fn update(&mut self, idx: usize, elm: &T) {
        self.array[idx] = elm.clone();
        
        let sqrt_idx = idx / Bucket::<T>::sqrt(self.n);
        if self.buckets[sqrt_idx] > *elm  {
            self.buckets[sqrt_idx] = elm.clone()
        }
    }

    fn find(&self, start: usize, end: usize) -> Option<&T> {
        let bucket_size = Bucket::<T>::sqrt(self.n);
        let b_start = start / bucket_size + if start % bucket_size == 0 {0} else {1};
        let b_end = end / bucket_size;

        // println!("start = {}, end = {}", start, end);
        // println!("{}, {}, {}", b_start, b_end, bucket_size);

        let min1 = self.array[start..b_start*bucket_size].iter().fold(None, |min,x| match min {
            None => Some(x),
            Some(y) => Some(if x < y { x } else { y }),
        });

        let min2 = self.buckets[b_start..b_end].iter().fold(None, |min, x| match min {
            None => Some(x),
            Some(y) => Some(if x < y {x} else {y}),
        });

        let min3 = self.array[b_end*bucket_size..end].iter().fold(None, |min,x| match min {
            None => Some(x),
            Some(y) => Some(if x < y { x } else { y }),
        });

        // println!("{:?}, {:?}, {:?}", min1, min2, min3);

        return vec![min1, min2, min3].into_iter().fold(None, |min, x| match min {
            None => x,
            Some(y) => {
                if let Some(xref) = x {
                    Some(if xref < y {xref} else {y})
                } else {
                    Some(y)
                }
            },
        });
    }

    // fn min(iter: IntoIterator) -> Option<T> {
    //     iter.fold(None, |min, x| match min {
    //         None => Some(x),
    //         Some(y) => Some(if x < y {x} else {y}),
    //     });
    //
    // }
}

fn main() {
    let sample = [1,3,2,6,7,4,3,0,8,4,3,5,9,2,1,5];

    let mut bucket = Bucket::new(sample.len(), &1000);

    for (i, elm) in sample.iter().enumerate() {
        bucket.update(i, elm);
    }

    // print
    println!("{:?}", bucket);

    println!("0-2 min {:?}", bucket.find(0, 2));
    println!("1-6 min {:?}", bucket.find(1, 6));
    println!("8-11 min {:?}", bucket.find(8, 11));
    println!("0-16 min {:?}", bucket.find(0, 16));
}
