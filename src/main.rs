const CARDCOUNT:usize = 12;

const CARDS:[[i32;4];CARDCOUNT] = [
	[0, 0, 0, 0], //0
	[1, 0, 0, 0], //1
	[0, 1, 0, 0], //2
	[0, 0, 1, 0], //3
	[1, 1, 0, 0], //4
	[1, 0, 1, 0], //5
	[0, 1, 1, 0], //6
	[0, 0, 1, 1], //7
	[1, 1, 1, 0], //8
	[1, 0, 1, 1], //9
	[0, 1, 1, 1], //10
	[1, 1, 1, 1],  //11
];



fn compatible(x:[i32;4],y:[i32;4]) -> bool {
	return 
		(x[0]>=y[0] && x[1]>=y[1]&&x[2]>=y[2]&&x[3]>=y[3]) || 
		(x[0]<=y[0] && x[1]<=y[1]&&x[2]<=y[2]&&x[3]<=y[3]);
}


#[derive(Copy, Clone, Debug)]
struct State {
	top : i32,
	left : i32,
	right : i32,	
	playable : [i32;CARDCOUNT],
	playable_count: usize
}

fn init_state() -> State {
	let mut result = State {
		top: -1,
		left: -1,
		right: -1,
		playable: [0;CARDCOUNT],
		playable_count: CARDCOUNT 
	};

	for i in 0..CARDCOUNT {
		result.playable[i]=i as i32;
	}
	return result;
}

fn gen_starting_decks() -> ([State;CARDCOUNT*CARDCOUNT],usize) {
	let mut result = [init_state();CARDCOUNT*CARDCOUNT];
	let mut states_used:usize = 0;
	for i  in 0..CARDCOUNT {
		for j in (i+1)..CARDCOUNT {	
			let mut r = result[states_used];				
			r.top = -1;
			r.left = i as i32;
			r.right = j as i32;
			for k in 0..(CARDCOUNT-2) {
				r.playable[k] = k as i32;
				if k >= j-1 {
					r.playable[k] += 2;
				} else if k >= i {
					r.playable[k] += 1;
				}
			}
			r.playable[CARDCOUNT-2]=-1;
			r.playable[CARDCOUNT-1]=-1;
			r.playable_count = CARDCOUNT-2;

			result[states_used] = r;
			states_used+=1;
		}
	}

	return (result,states_used);
}

fn solidify( s : State ) -> ([State;CARDCOUNT*CARDCOUNT],usize) {

	let mut result = [s;CARDCOUNT*CARDCOUNT];

	if s.playable_count==0 {
		result[0]=s;
		return (result,1);
	}

	if s.left == -1 && s.right == -1 {
		return gen_starting_decks()
	}

	for i in 0..s.playable_count {
		let toplay = s.playable[i];		
		let mut t : State = State {
			top : s.top,
			left : s.left,
			right : s.right,
			playable : [-1;CARDCOUNT],
			playable_count : s.playable_count-1
		};
		for j in 0..i {
			t.playable[j] = s.playable[j];
		}
		for j in (i+1)..s.playable_count {
			t.playable[j-1] = s.playable[j];
		}
		if s.left == -1 {
			t.left = toplay;
			t.right = s.right;
		} else {
			t.left = s.left;
			t.right = toplay;
		}
		result[i] = t;

	}
	return (result,s.playable_count);
}

fn can_play_left(s : State, compatibility: [[i32;CARDCOUNT];CARDCOUNT]) -> bool {
	if s.top == -1 {
		return true;
	}

	if s.left == -1 {
		return false;
	}

	if compatibility[s.left as usize][s.top as usize] == 1 {
		return true;
	} else {
		return false;
	}

}

fn can_play_right(s : State, compatibility: [[i32;CARDCOUNT];CARDCOUNT]) -> bool {
	if s.top == -1 {
		return true;
	}

	if s.right == -1 {
		return false;
	}

	if compatibility[s.right as usize][s.top as usize] == 1 {
		return true;
	} else {
		return false;
	}
}

fn play_left(s : State) -> State {
	let t: State = State {
		top: s.left,
		left: -1,
		right: s.right,
		playable: s.playable.clone(),
		playable_count: s.playable_count
	};
	return t;
}

fn play_right(s : State) -> State {
	let t: State = State {
		top: s.right,
		left: s.left,
		right: -1,
		playable: s.playable.clone(),
		playable_count: s.playable_count
	};
	return t;
}

fn e(s :State, compatibility: [[i32;CARDCOUNT];CARDCOUNT]) -> f64 {

	if s.left == -1 && s.right == -1 {
		return 1 as f64;
	}

	let mut left_score : f64 = 0 as f64;
	if can_play_left(s,compatibility) {
		let t = play_left(s);
		let (possibilities,l) = solidify(t);
		//println!("{} -> {}\n", s.playable_count, possibilities[0].playable_count);
		for i  in 0..l {
			let p = possibilities[i];
			left_score += e(p,compatibility) / (l as f64);
		}
	}

	let mut right_score : f64 = 0 as f64;
	if can_play_right(s,compatibility) {
		let t = play_right(s);

		let (possibilities,l) = solidify(t);
		for i in 0..l {
			let p = possibilities[i];
			right_score += e(p,compatibility) / (l as f64);
		}
	}
	if left_score>right_score {
		return left_score;
	} else {
		return right_score;
	}
}


fn gen_compatibility_matrix() -> [[i32;CARDCOUNT];CARDCOUNT]  {
	let mut compatibility:[[i32;CARDCOUNT];CARDCOUNT] 
		= [[0;CARDCOUNT];CARDCOUNT];
	
	for i in 0..CARDCOUNT {
		compatibility[i][i] = 1;
		for j in (i + 1)..CARDCOUNT {
			compatibility[i][j] = if compatible(CARDS[i], CARDS[j]) { 1 } else { 0 };
			compatibility[j][i] = compatibility[i][j];
		}
	}
	return compatibility;
}
fn do_sim(){

	let compatibility = gen_compatibility_matrix();
	let init = init_state();
	let (starting_states,states_used) = solidify(init);

	let mut probs : [f64;CARDCOUNT*CARDCOUNT] = [0 as f64;CARDCOUNT*CARDCOUNT];

	for i in 0..states_used {
		probs[i] = e(starting_states[i],compatibility);
		let left = starting_states[i].left;
		let right = starting_states[i].right;
		println!("({},{}) -> {}",left,right,100.0*probs[i]);
	}

	let mut average : f64 = 0 as f64;
	for i  in 0..states_used {
		average += probs[i] / (states_used as f64);
	}
	println!("total average = {}", 100.0*average);
}


fn main() {
	do_sim();
}