//声明节点状态机
#[derive(Debug)]
struct PersistentState<T>{
    current_term:T,
    vote_for:Option<String>,
    log:Vec<String>,
    commit_index:Option<T>,
    last_applied:Option<T>
}
#[derive(Debug)]
struct LeaderState<T>{
    next_index:T,
    match_index:T
}
#[derive(Debug)]
struct Follower<T>{
    persistent_state:PersistentState<T>
}
#[derive(Debug)]
struct Candidate<T>{
    persistent_state:PersistentState<T>
}
#[derive(Debug)]
struct Leader<T>{
    persistent_state:PersistentState<T>,
    leader_state:LeaderState<usize>
}

impl<T> Follower<T>{
    fn new(init_value:T)->Self{
        Follower{
            persistent_state:PersistentState{
                current_term:init_value,
                vote_for:None,
                log:Vec::new(),
                commit_index:None,
                last_applied:None
            }
        }
    }
}

impl<T> Follower<T>{
    fn upgrade(self)->Candidate<T>{
        Candidate{
            persistent_state:self.persistent_state
        }
    }
}

impl<T> Candidate<T>{
    fn upgrade(self)->Leader<T>{
        Leader{
            leader_state:LeaderState{
                next_index:self.persistent_state.log.len(),
                match_index:self.persistent_state.log.len() - 1
            },
            persistent_state:self.persistent_state
        }
    }

    fn downgrade(self)->Follower<T>{
        Follower{
            persistent_state:self.persistent_state
        }
    }
}

impl<T> Leader<T>{
    fn downgrade(self)->Follower<T>{
        Follower { 
            persistent_state:self.persistent_state 
        }
    }
}

fn main(){
    println!("{:?}",Follower::new(0));
}