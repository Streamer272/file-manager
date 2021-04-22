pub struct Difference {
    differences: String,
    difference_indexes: usize,
}

pub struct FileDifferences {
    added: Vec<Difference>,
    modified: Vec<Difference>,
    deleted: Vec<Difference>
}


// algorithm choices
// longest common subsequence O(2^n || n)
// Eugene Myer - http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.4.6927
// Paul Heckel - http://documents.scribd.com/docs/10ro9oowpo1h81pgh1as.pdf


/*
1:
1234
56

2:
123
4321
56
 */
