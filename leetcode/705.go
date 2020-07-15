import (
	"fmt"
)

const BASE_PRIME = 13

func findElement(slice []int, element int) int {
	for i := 0; i < len(slice); i++ {
		if slice[i] == element {
			return i;
		}
	}
	return -1;
}

func removeIndex(slice []int, index int) []int {
	slice[index] = slice[len(slice)-1];
	slice[len(slice)-1] = 0;
	slice = slice[:len(slice)-1];
	return slice;
}

func removeElement(slice []int, element int) []int {
    for true {
        index := findElement(slice, element);
        if index == -1 {
            return slice;
        } else {
            slice = removeIndex(slice, index);
        }
    }
    return slice;
}


type MyHashSet struct {
    hashTable [BASE_PRIME][]int
}


/** Initialize your data structure here. */
func Constructor() MyHashSet {
    t := MyHashSet{}
    return t
}

func hash(key int) int {
    return key % BASE_PRIME
}

func (this *MyHashSet) Add(key int)  {
    hashed := hash(key)
    this.hashTable[hashed] = append(this.hashTable[hashed], key)
}


func (this *MyHashSet) Remove(key int)  {
    hashed := hash(key)
    this.hashTable[hashed] = removeElement(this.hashTable[hashed], key)
}


/** Returns true if this set contains the specified element */
func (this *MyHashSet) Contains(key int) bool {
    hashed := hash(key)
    index := findElement(this.hashTable[hashed], key)
    if index == -1 {
        return false;
    } else {
        return true;
    }
}


/**
 * Your MyHashSet object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Add(key);
 * obj.Remove(key);
 * param_3 := obj.Contains(key);
 */
