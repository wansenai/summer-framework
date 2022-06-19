//声明节点状态机
struct Follower<T,U>{
    CurrentTerm:T,
    VoteFor:Option<U>,
    Log:Vec<U>,
    CommitIndex:Option<T>,
    LastApplied:Option<T>
}
struct Candidate<T,U>{
    CurrentTerm:T,
    VoteFor:Option<U>,
    Log:Vec<U>,
    CommitIndex:Option<T>,
    LastApplied:Option<T>
}
struct Leader<T,U>{
    CurrentTerm:T,
    VoteFor:Option<U>,
    Log:Vec<U>,
    CommitIndex:Option<T>,
    LastApplied:Option<T>,
    NextIndex:T,
    MatchIndex:T
}
trait New{
    fn new()->Self;
}
trait StateChange{
    fn upgrade(&mut self)->Self;
    fn downgrade(&mut self)->Self;
}

impl<T,U> New for Follower<T,U>{
    type Output = Follower<T,U>;
    fn new()->Self{
        Follower{
            CurrentTerm:0,
            VoteFor:None,
            Log:Vec::new(),
            CommitIndex:None,
            LastApplied:None
        }
    }
}