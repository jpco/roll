extern crate rand;

use std::iter::Iterator;
use std::collections::VecDeque;
use std::borrow::Borrow;

use rand::distributions::{Distribution, Uniform};

fn roll_dice(dct: u32, dtype: u32) -> u32 {
    // TODO: determine if/when using a Uniform is faster than the simple method
    let mut rng = rand::thread_rng();
    let range = Uniform::new_inclusive(1, dtype);

    let mut accum = 0;
    for _ in 0..dct {
        accum += range.sample(&mut rng);
    }
    accum
}

enum RState {
    Normal,
    Count,
    D,
    Type,
}

pub struct Rollerator<I> {
    src: I,               // the data source
    tmp_in: Option<char>, // temporary single-char input buffer
    state: RState,        // the state of the Rollerator
    dct: u32,             // current die count
    dtype: u32,           // current die type
    buf: VecDeque<char>,  // the buffer of output chars
}

impl<I: Iterator<Item = char>> Rollerator<I> {
    fn new(src: I) -> Rollerator<I> {
        Rollerator {
            src: src,
            tmp_in: None,
            state: RState::Normal,
            dct: 0,
            dtype: 0,
            buf: VecDeque::new(),
        }
    }

    fn reset(&mut self) -> Option<char> {
        self.state = RState::Normal;
        self.dct = 0;
        self.dtype = 0;
        self.buf.pop_front()
    }

    fn roll(&mut self) -> Option<char> {
        if self.dtype > 0 {
            self.buf = roll_dice(self.dct, self.dtype)
                .to_string()
                .chars()
                .collect();
        }
        return self.reset();
    }

    fn get_one(&mut self) -> Option<char> {
        if let Some(tmp) = self.tmp_in.take() {
            return Some(tmp);
        }
        if let Some(src) = self.src.next() {
            return Some(src);
        }
        None
    }

    fn build_roll(&mut self) -> Option<char> {
        loop {
            let c = self.get_one();
            if c == None {
                return match &self.state {
                    &RState::Type => self.roll(),
                    _ => self.reset(),
                };
            }
            let c = c.unwrap();

            match (&self.state, c) {
                (&RState::Normal, c @ '0'...'9') | (&RState::Count, c @ '0'...'9') => {
                    self.state = RState::Count;
                    self.dct = self.dct * 10 + c.to_digit(10).unwrap();
                }
                (&RState::Normal, 'd') => {
                    self.state = RState::D;
                    self.dct = 1;
                }
                (&RState::Normal, c) => {
                    return Some(c);
                }

                (&RState::Count, 'd') => self.state = RState::D,

                (&RState::D, c @ '0'...'9') | (&RState::Type, c @ '0'...'9') => {
                    self.state = RState::Type;
                    self.dtype = self.dtype * 10 + c.to_digit(10).unwrap();
                }

                (&RState::Type, c) => {
                    self.tmp_in = Some(c);
                    return self.roll();
                }

                (_, c) => {
                    self.tmp_in = Some(c);
                    return self.reset();
                }
            }
            self.buf.push_back(c);
        }
    }
}

impl<I: Iterator<Item = char>> Iterator for Rollerator<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // If there are pending chars in the buffer, return one
        if let Some(c) = self.buf.pop_front() {
            return Some(c);
        }

        // Build up the buffer and return its first element
        self.build_roll()
    }
}

pub trait Iterollable {
    fn roll(self) -> Rollerator<Self>
    where
        Self: Sized + Iterator<Item = char>;
}

impl<T> Iterollable for T
where
    T: Iterator<Item = char>,
{
    fn roll(self) -> Rollerator<Self> {
        Rollerator::new(self)
    }
}

pub trait Rollable {
    fn rolled(&self) -> String;
}

impl<T> Rollable for T
where
    T: Borrow<str>,
{
    fn rolled(&self) -> String {
        self.borrow().chars().roll().collect()
    }
}
