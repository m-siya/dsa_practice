use std::collections::{BinaryHeap, HashMap};

type UserId = i32;
type TweetId = i32;
type Time = usize;

mod fixed_buf {
    use std::collections::VecDeque;
    // Just in case I wanna change it later to a stack-based custom ring-buffer,
    // I'm abstracting over the implementation..
    pub struct FixedBuf<T, const N: usize> {
        inner: VecDeque<T>,
    }

    impl<T, const N: usize> FixedBuf<T, N> {
        pub fn new() -> FixedBuf<T, N> {
            Self {
                inner: VecDeque::with_capacity(N),
            }
        }

        pub fn push(&mut self, elem: T) {
            if self.inner.len() == N {
                self.inner.pop_front();
            }
            self.inner.push_back(elem);
        }

        pub fn get(&self, i: usize) -> Option<&T> {
            self.inner.get(i)
        }

        pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
            self.inner.get_mut(i)
        }

        pub fn iter(&self) -> FixedBufIter<'_, T, N> {
            FixedBufIter {
                buf: self,
                curr_idx: (self.inner.len() - 1) as isize,
            }
        }

        pub fn len(&self) -> usize {
            self.inner.len()
        }
    }

    pub struct FixedBufIter<'a, T, const N: usize> {
        buf: &'a FixedBuf<T, N>,
        curr_idx: isize,
    }

    impl<'a, T, const N: usize> Iterator for FixedBufIter<'a, T, N> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.curr_idx < 0 {
                return None;
            }

            let curr_elem = self.buf.get(self.curr_idx as usize);
            self.curr_idx -= 1;
            curr_elem
        }

        // Can be relied upon for correctness.
        fn size_hint(&self) -> (usize, Option<usize>) {
            let buf_len = self.buf.len();
            let size_remaining = (self.curr_idx + 1) as usize;
            (size_remaining, Some(size_remaining))
        }
    }

    impl<'a, T: PartialOrd, const N: usize> PartialOrd for FixedBufIter<'a, T, N> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            let lhs = self.buf.get(
                self.curr_idx
                    .try_into()
                    .expect("partial_cmp called for an empty iterator"),
            );
            let rhs = other.buf.get(
                other
                    .curr_idx
                    .try_into()
                    .expect("partial_cmp called for an empty iterator"),
            );
            lhs?.partial_cmp(rhs?)
        }
    }

    impl<'a, T: PartialOrd, const N: usize> Ord for FixedBufIter<'a, T, N> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(other)
                .expect("cmp called for an empty iterator")
        }
    }

    impl<'a, T: PartialEq, const N: usize> PartialEq for FixedBufIter<'a, T, N> {
        fn eq(&self, other: &Self) -> bool {
            let lhs = self.buf.get(
                self.curr_idx
                    .try_into()
                    .expect("Eq called for an empty iterator"),
            );
            let rhs = other.buf.get(
                other
                    .curr_idx
                    .try_into()
                    .expect("Eq called for an empty iterator"),
            );
            (|| Some(lhs?.eq(rhs?)))().unwrap_or(false)
        }
    }

    impl<'a, T: PartialEq, const N: usize> Eq for FixedBufIter<'a, T, N> {}
}

use fixed_buf::FixedBuf;
struct Twitter {
    users: HashMap<UserId, User>,
    curr_time: Time,
}

struct User {
    tweets: FixedBuf<Tweet, 10>,
    follows: Vec<UserId>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            tweets: FixedBuf::<Tweet, 10>::new(),
            follows: vec![],
        }
    }
}

struct Tweet {
    timestamp: Time,
    tweet: TweetId,
}

impl PartialOrd for Tweet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.timestamp.partial_cmp(&other.timestamp)
    }
}

impl Ord for Tweet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl Eq for Tweet {}

impl PartialEq for Tweet {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Twitter {
    // Current design optimises posting a tweet rather than reading tweets.
    // IRL, most people read tweets a LOT more than they write one,
    // (although the small vocal minority that does, has a LOT of subscribers, so there's no clear winner).
    // To accomodate this, one approach we could take is a publisher-subscriber architecture.
    // In this, posting a tweet is more expensive, as instead of just posting, it also adds the tweet,
    // to the newsfeed of all of their followers.
    // So our data-structures would look a little different.
    // User { newsfeed: FixedBuf<Tweet>, followers: Vec<UserId>};
    // instead of
    // User { tweets  : FixedBuf<Tweet>, follows  : Vec<UserId>};
    // This way, getting the newsfeed is as simple as just cloning the newsfeed and returning it.

    // As I'm writing this out, I'm realising that this approach will actually only be
    // O(num_followers) and LeetCode says we only have at max 500 users in our application, so this will
    // be pretty fast.
    // (and also actually easier to code for, so I'm regretting not going for the above approach).
    // Will prolly A-B test a TwitterV2 with the above arch.

    // Note that this appraoch however requires additional use of space for storing more tweets per-user, (since they may unfollow a
    // user and hence their newsfeed must now include older tweets to bring the count back to 10),
    // and same tweet_id will be stored for multiple users. Storing tweet_id is not that bad, it's just i32.
    // We don't want to store the ENTIRE newsfeed for each user just because they may unfollow someone.
    // As a pessimistic measure, we can store the last 10 tweets for each user they follow, however now it's starting to approach
    // the same code complexity as this soln.

    fn new() -> Self {
        const AVG_EXPECTED_USERS: usize = 50;
        Self {
            users: HashMap::with_capacity(AVG_EXPECTED_USERS),
            curr_time: 0,
        }
    }

    fn post_tweet(&mut self, user_id: UserId, tweet_id: TweetId) {
        let user = self.users.entry(user_id).or_default();
        user.tweets.push(Tweet {
            timestamp: self.curr_time,
            tweet: tweet_id,
        });
        self.curr_time += 1;
    }

    fn get_news_feed(&self, user_id: UserId) -> Vec<i32> {
        // basically the "merge k sorted list" problem.
        let mut latest_k_tweets: Vec<TweetId> = Vec::with_capacity(10);
        let user = match self.users.get(&user_id) {
            Some(user) => user,
            None => return vec![],
        };

        let mut prior_q = BinaryHeap::with_capacity(user.follows.len() + 1);
        for &followee in user.follows.iter() {
            let followee_tweets = &self.users.get(&followee)
                .expect("A followee found in a twitter user that doesn't exist (anamoly found in get_news_feed())")
                .tweets;
            if followee_tweets.len() != 0 {
                prior_q.push(followee_tweets.iter());
            }
        }

        if user.tweets.len() != 0 {
            prior_q.push(user.tweets.iter());
        }

        while let Some(mut latest_tweet) = prior_q.pop() {
            if latest_k_tweets.len() >= 10 {
                break;
            }

            latest_k_tweets.push(
                latest_tweet
                    .next()
                    .expect("latest_tweet should not have been an empty iterator")
                    .tweet,
            );

            if latest_tweet.size_hint().0 != 0 {
                prior_q.push(latest_tweet);
            }
        }

        latest_k_tweets
    }

    fn follow(&mut self, follower_id: UserId, followee_id: UserId) {
        let followees = &mut self.users.entry(follower_id).or_default().follows;
        if !followees.contains(&followee_id) {
            followees.push(followee_id);
        }
        self.users.entry(followee_id).or_default();
    }

    // In the second arch, unfollow() will be tricky, since we'd need to go thru the
    // feed of the follower_id follower and remove all the tweets belonging to the
    // followee_id user. However that also means that now the last 10 tweets will
    // have older tweets as well. So, we'll probably need to rebuild the newsfeed
    // on every unfollow.
    fn unfollow(&mut self, follower_id: UserId, followee_id: UserId) {
        if let Some(follower) = self.users.get_mut(&follower_id) {
            if let Some(followee_idx) = follower
                .follows
                .iter()
                .position(|&followee| followee == followee_id)
            {
                follower.follows.swap_remove(followee_idx);
            }
        }
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */

fn main() {
    println!("Hello, world!");
}
