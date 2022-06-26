//声明节点状态机
#[derive(Debug)]
enum State{
    Follower,
    Candidate,
    Leader
}
#[derive(Debug)]
pub struct Node{
    id:String,
    state:State,
    current_term:usize,
    vote_for:Option<String>,
    log:Vec<(usize,String)>,
    commit_index:Option<usize>,
    last_applied:Option<usize>,
    next_index:Option<usize>,
    match_index:Option<usize>
}

impl Node{
    fn new(id:String)->Self{
        Self{
            id,
            state:State::Follower,
            current_term:1,
            vote_for:None,
            log:Vec::new(),
            commit_index:None,
            last_applied:None,
            next_index:None,
            match_index:None
        }
    }

    fn upgrade(&mut self){
        match self.state{
            State::Follower=>{
                self.state=State::Candidate;
            },
            State::Candidate=>{
                let len=self.log.len();
                self.state=State::Leader;
                self.next_index=Some(len);
                self.match_index=Some(len-1);
            }
            State::Leader=>{
                ()
            }
        }
    }

    fn downgrade(&mut self){
        match self.state{
            State::Follower=>{
                ()
            },
            State::Candidate=>{
                self.state=State::Follower;
            },
            State::Leader=>{
                self.next_index=None;
                self.match_index=None;
                self.state=State::Candidate;
            }
        }
    }

    fn id(&self)->String{
        self.id
    }

    fn set_id(&mut self,id:String){
        self.id=id;
    }

    fn term(&self)->usize{
        self.current_term
    }

    fn set_term(&mut self,term:usize){
        self.current_term=term;
    }

    fn vote(&self)->String{
        self.vote_for.unwrap_or("".to_string())
    }

    fn next_index(&self)->usize{
        self.next_index.unwrap_or(0)
    }

    fn prev_log_index(&self)->usize{
        self.next_index.unwrap_or(1) - 1
    }

    fn prev_log_term(&self)->usize{
        self.log.last().unwrap_or(&(0,"".to_string())).0
    }

    fn commit_index(&self)->Option<usize>{
        self.commit_index
    }
}