//! Copy-on-write typed history maps. The serialized format is exactly the
//! original map. Dirty tracking is an optimization, never a caller-supplied
//! journal or an alternate authority. Untracked mutable access marks the
//! entire map dirty, conservatively falling back to a full table rewrite.
use serde::{Serialize,Serializer,Deserialize,Deserializer};
use std::{collections::{BTreeMap,BTreeSet},ops::{Deref,DerefMut},sync::Arc};
#[derive(Clone,Debug)]
pub(crate) enum Changes{Keys(BTreeSet<String>),Full}
#[derive(Clone,Debug)]
pub struct SharedMap<V>{data:Arc<BTreeMap<String,V>>,changes:Changes}
impl<V> Default for SharedMap<V>{
    fn default()->Self{Self{data:Arc::new(BTreeMap::new()),changes:Changes::Keys(BTreeSet::new())}}
}
impl<V:PartialEq> PartialEq for SharedMap<V>{fn eq(&self,other:&Self)->bool{self.data==other.data}}
impl<V:Eq> Eq for SharedMap<V>{}
impl<V:Serialize> Serialize for SharedMap<V>{
    fn serialize<S:Serializer>(&self,s:S)->std::result::Result<S::Ok,S::Error>{self.data.as_ref().serialize(s)}
}
impl<'de,V:Deserialize<'de>> Deserialize<'de> for SharedMap<V>{
    fn deserialize<D:Deserializer<'de>>(d:D)->std::result::Result<Self,D::Error>{
        let data=BTreeMap::deserialize(d)?;
        Ok(Self{data:Arc::new(data),changes:Changes::Keys(BTreeSet::new())})
    }
}
impl<V> Deref for SharedMap<V>{type Target=BTreeMap<String,V>;fn deref(&self)->&Self::Target{&self.data}}
impl<V:Clone> DerefMut for SharedMap<V>{
    fn deref_mut(&mut self)->&mut Self::Target{self.changes=Changes::Full;Arc::make_mut(&mut self.data)}
}
impl<'a,V> IntoIterator for &'a SharedMap<V>{
    type Item=(&'a String,&'a V);type IntoIter=std::collections::btree_map::Iter<'a,String,V>;
    fn into_iter(self)->Self::IntoIter{self.data.iter()}
}
impl<'a,V:Clone> IntoIterator for &'a mut SharedMap<V>{
    type Item=(&'a String,&'a mut V);type IntoIter=std::collections::btree_map::IterMut<'a,String,V>;
    fn into_iter(self)->Self::IntoIter{self.deref_mut().iter_mut()}
}
impl<V:Clone> IntoIterator for SharedMap<V>{
    type Item=(String,V);type IntoIter=std::collections::btree_map::IntoIter<String,V>;
    fn into_iter(self)->Self::IntoIter{Arc::unwrap_or_clone(self.data).into_iter()}
}
impl<V> From<BTreeMap<String,V>> for SharedMap<V>{
    fn from(data:BTreeMap<String,V>)->Self{Self{data:Arc::new(data),changes:Changes::Full}}
}
impl<V> FromIterator<(String,V)> for SharedMap<V>{
    fn from_iter<T:IntoIterator<Item=(String,V)>>(iter:T)->Self{BTreeMap::from_iter(iter).into()}
}
impl<V> SharedMap<V>{
    pub fn is_empty(&self)->bool{self.data.is_empty()}
    pub(crate) fn changes(&self)->&Changes{&self.changes}
    pub(crate) fn mark_clean(&mut self){self.changes=Changes::Keys(BTreeSet::new());}
    fn touch(&mut self,key:&str){
        if let Changes::Keys(keys)=&mut self.changes{
            keys.insert(key.to_owned());
            // Bound tracking memory independently of world size.
            if keys.len()>4096{self.changes=Changes::Full;}
        }
    }
}
impl<V:Clone> SharedMap<V>{
    pub fn insert(&mut self,key:String,value:V)->Option<V>{
        self.touch(&key);Arc::make_mut(&mut self.data).insert(key,value)
    }
    pub fn get_mut(&mut self,key:&str)->Option<&mut V>{
        if !self.data.contains_key(key){return None;}
        self.touch(key);Arc::make_mut(&mut self.data).get_mut(key)
    }
    pub fn remove(&mut self,key:&str)->Option<V>{
        if !self.data.contains_key(key){return None;}
        self.touch(key);Arc::make_mut(&mut self.data).remove(key)
    }
    pub fn clear(&mut self){
        self.changes=Changes::Full;self.data=Arc::new(BTreeMap::new());
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn clone_and_mutation_preserve_old_value_and_serde_shape(){
        let mut a=SharedMap::default();a.insert("x".into(),1u64);a.mark_clean();
        let mut b=a.clone();assert!(Arc::ptr_eq(&a.data,&b.data));
        *b.get_mut("x").unwrap()=2;b.insert("y".into(),3);
        assert_eq!(a["x"],1);assert_eq!(b["x"],2);
        assert!(matches!(b.changes(),Changes::Keys(keys) if keys.len()==2));
        assert_eq!(serde_json::to_value(&b).unwrap(),serde_json::json!({"x":2,"y":3}));
        b.retain(|_,v|*v>2);assert!(matches!(b.changes(),Changes::Full));
        assert_eq!(a.len(),1);
    }
}
