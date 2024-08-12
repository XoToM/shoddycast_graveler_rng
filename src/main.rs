
//	Some of the imports below are here to allow for easily changing out the random number generator this program uses.
use std::ops::RangeFull;
use rand::{rngs::{StdRng, ThreadRng}, RngCore, SeedableRng};
use rand_isaac::Isaac64Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::time::Instant;
use rand_xoshiro::{Xoroshiro128PlusPlus};

const BLOCK_SIZE:usize = 128;				//	The calculations are split into batches. This is the number of calculations per batch. Must be lower than the TOTAL_COUNT.
const THREAD_COUNT:usize = 50000;			//	Multiple threads may be used to speed up the calculations. The bigger this number, the less calculations each thread will have to do, and the more threads may be used (this won't run more threads than your computer can handle though, so setting this value too high might actually decrease performance). Must be lower than the TOTAL_COUNT. You can also set this to 1 to disable multi-threading.

const TOTAL_COUNT:usize = 1_000_000_000;	//	Number of simulations to run. You can increase/decrease this to change how many dice rolls will happen.

//	To achieve the performance in the proof video you should run this code with the "cargo run --release" command. The normal "cargo run" command disables most optimizations, as it is intended for debugging the code.

fn main() {

	println!("Searching {} iterations ({}x231 random picks)", TOTAL_COUNT,TOTAL_COUNT);
    let now = Instant::now();

	let maximum = (0..THREAD_COUNT).into_par_iter().map(|_|{

		//	The random number generator is the most important part of this program, and choosing the right one is important for performance.

		//	The proof was ran with the fastrand library which is an implementation of the Wyhash rng algorihm, which is one of the fastest rng algorithms which still provides highly random numbers.
		//	You can switch out the line below with the commented out lines following it to try out different rng algorithms.

		let mut rng = FastRand(fastrand::Rng::new());		//	Fastest rng with decently high quality. Uses the wyhash algorithm
		//let mut rng = RandGens(Xoroshiro128PlusPlus::from_entropy());	//	Slightly slower than the above, but has comparable quality. Uses the Xoroshiro 128 Plus Plus algorithm.

		//let mut rng = RandGens(Isaac64Rng::from_entropy());	//	Slower rng, but uses information from other parts of the computer to achieve more randomness. This uses the ISAAC 64 algorithm.
		//let mut rng = RandGens(StdRng::from_entropy());	//	Slowest rng, but also uses other data for better randomness. This uses the ChaCha block cipher for its random values.

		(0..(TOTAL_COUNT/THREAD_COUNT/BLOCK_SIZE)).into_iter().map(|_| {
					let res = do_random_block(&mut rng);
					res
				}).max().unwrap()
			}).max().unwrap();

	println!("Found the largest number {} after {:.3?}",maximum, now.elapsed());
}

fn do_random_block<R: RandNumGen>(rng: &mut R) -> u32{
	//	I slightly changed the algorithm here. Instead of running the random function once per every dice roll, I'm calling the random function once per 32 dice rolls.
	//	The random function only returns one number though, which is why I then have to do some fancy bit manipulation to check for the correct values.
	//	I'm batching these random calls together too. The code below calls the random function 8*BLOCK_SIZE times, meaning that I am doing all 231 dice rolls BLOCK_SIZE times here.
	//	This is also a small speedup, as it allows the cpu to run the code faster, and also allows the rust compiler to optimize the code more.
	//	I've hard coded the masks needed for bit manipulation here, meaning that this program is hardcoded to run 231 dice rolls per iteration.
	let rng_block: [[u64;8];BLOCK_SIZE] = core::array::from_fn(|_|{
		let mut arr = core::array::from_fn(|_|rng.gen());
		arr[7] |= 0b1111_1111111111_1111111111_1111111111_1111111111_1111110000_0000000000;
		arr
	});


	let res = rng_block.iter().map(|b|{
		b.into_iter().map(|v|{
			let mut val = *v;
			val |= (v & 0b01010101_01010101_01010101_01010101_01010101_01010101_01010101_01010101) << 1;
			val |= 0b01010101_01010101_01010101_01010101_01010101_01010101_01010101_01010101;
			u64::count_zeros(val)
		}).sum()
	}).max().unwrap();
	return res;
}


//	--- Definitions for types which allow switching out random number generators ---
trait RandNumGen{
	fn gen(&mut self)->u64;
}

struct FastRand(fastrand::Rng);

impl RandNumGen for FastRand{
	fn gen(&mut self)->u64 {
		self.0.u64(RangeFull)
	}
}
struct RandGens<T>(T);

impl<T: RngCore> RandNumGen for RandGens<T>{
	fn gen(&mut self)->u64 {
		self.0.next_u64()
	}
}