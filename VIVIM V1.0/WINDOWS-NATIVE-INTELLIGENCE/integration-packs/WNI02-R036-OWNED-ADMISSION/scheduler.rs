//! Deterministic resource admission proposal; never starts a process.
//! Host must reserve atomically and revalidate current grant before launch.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Snapshot{pub revision:u64,pub observed_ms:u64,pub memory_available:u64,pub disk_available:u64,pub logical_cpus:u32,pub cpu_busy_bp:Option<u16>}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Budget{pub max_age_ms:u64,pub memory_reserve:u64,pub disk_reserve:u64,pub cpu_reserve:u32,pub max_parallel:u32,pub busy_ceiling_bp:u16}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Work{pub id:u64,pub memory:u64,pub disk:u64,pub cpus:u32,pub priority:u8,pub enqueued_ms:u64}
#[derive(Debug,Clone,Copy,PartialEq,Eq,Default)]
pub struct Reserved{pub memory:u64,pub disk:u64,pub cpus:u32,pub tasks:u32}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Block{Invalid,Stale,UnknownCpu,Busy,Memory,Disk,Cpu,Parallel}
#[derive(Debug,PartialEq,Eq)]pub struct Plan{pub revision:u64,pub admitted:Vec<u64>,pub deferred:Vec<(u64,Block)>,pub reservations:Reserved}
pub fn plan(s:Snapshot,b:Budget,already:Reserved,now:u64,work:&[Work])->Result<Plan,Block>{
 if s.revision==0||s.logical_cpus==0||b.max_age_ms==0||b.max_age_ms>60000||b.max_parallel==0||b.max_parallel>64||b.busy_ceiling_bp>10000||work.len()>256{return Err(Block::Invalid);}
 if now.checked_sub(s.observed_ms).is_none_or(|age|age>b.max_age_ms){return Err(Block::Stale);}
 let mut seen=std::collections::HashSet::new();
 if work.iter().any(|w|w.id==0||!seen.insert(w.id)||w.cpus==0||w.memory==0||w.enqueued_ms>now){return Err(Block::Invalid);}
 let busy=s.cpu_busy_bp.ok_or(Block::UnknownCpu)?;
 if busy>10000{return Err(Block::Invalid);}
 let mem=s.memory_available.saturating_sub(b.memory_reserve);
 let disk=s.disk_available.saturating_sub(b.disk_reserve);
 let cpus=s.logical_cpus.saturating_sub(b.cpu_reserve);
 let mut sorted=work.to_vec();sorted.sort_by_key(|w|(std::cmp::Reverse(w.priority),w.enqueued_ms,w.id));
 let mut p=Plan{revision:s.revision,admitted:vec![],deferred:vec![],reservations:already};
 for w in sorted{
  let r=p.reservations;
  let reason=if busy>=b.busy_ceiling_bp{Some(Block::Busy)}
   else if r.tasks>=b.max_parallel{Some(Block::Parallel)}
   else if w.memory>mem.saturating_sub(r.memory){Some(Block::Memory)}
   else if w.disk>disk.saturating_sub(r.disk){Some(Block::Disk)}
   else if w.cpus>cpus.saturating_sub(r.cpus){Some(Block::Cpu)}else{None};
  if let Some(reason)=reason{p.deferred.push((w.id,reason));continue;}
  p.reservations=Reserved{memory:r.memory+w.memory,disk:r.disk+w.disk,cpus:r.cpus+w.cpus,tasks:r.tasks+1};p.admitted.push(w.id);
 }
 Ok(p)
}
#[cfg(test)]mod tests{
 use super::*;
 fn s()->Snapshot{Snapshot{revision:1,observed_ms:100,memory_available:100,disk_available:100,logical_cpus:8,cpu_busy_bp:Some(1000)}}
 fn b()->Budget{Budget{max_age_ms:500,memory_reserve:20,disk_reserve:20,cpu_reserve:2,max_parallel:3,busy_ceiling_bp:9000}}
 fn w(id:u64,priority:u8)->Work{Work{id,memory:30,disk:30,cpus:2,priority,enqueued_ms:1}}
 #[test]fn priority_and_reservations(){let r=plan(s(),b(),Reserved::default(),100,&[w(3,1),w(2,2),w(1,2)]).unwrap();assert_eq!(r.admitted,vec![1,2]);assert_eq!(r.deferred,vec![(3,Block::Memory)]);assert_eq!(r.reservations.memory,60);}
 #[test]fn stale_unknown_and_busy(){assert_eq!(plan(s(),b(),Reserved::default(),601,&[]),Err(Block::Stale));assert_eq!(plan(Snapshot{cpu_busy_bp:None,..s()},b(),Reserved::default(),100,&[]),Err(Block::UnknownCpu));assert_eq!(plan(Snapshot{cpu_busy_bp:Some(9000),..s()},b(),Reserved::default(),100,&[w(1,1)]).unwrap().deferred,vec![(1,Block::Busy)]);}
 #[test]fn prior_reservations_and_invalid(){assert_eq!(plan(s(),b(),Reserved{memory:60,..Reserved::default()},100,&[w(1,1)]).unwrap().deferred,vec![(1,Block::Memory)]);assert_eq!(plan(s(),b(),Reserved::default(),100,&[w(1,1),w(1,2)]),Err(Block::Invalid));}
 #[test]fn no_overflow_and_disk(){let r=plan(Snapshot{disk_available:10,..s()},b(),Reserved::default(),100,&[w(1,1)]).unwrap();assert_eq!(r.deferred,vec![(1,Block::Disk)]);let r=plan(s(),b(),Reserved{memory:u64::MAX,..Reserved::default()},100,&[w(1,1)]).unwrap();assert_eq!(r.deferred,vec![(1,Block::Memory)]);}
}
