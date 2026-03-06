use rustlearn::prelude::*;
use rustlearn::datasets::iris;

use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

use rustlearn::ensemble::random_forest::Hyperparameters;
use rustlearn::metrics::accuracy_score;
use rustlearn::trees::decision_tree;

fn main() {
    let (data, target) = iris::load_data();

    let total_rows = 150; // for iris dataset 
    let train_ratio = 0.7; // 0.7 to split the data in 70/30 ratio
    let mut indices: Vec<usize> = {0..total_rows}.collect(); // Vector of indices [0 to 149]

    let mut rng = StdRng::seed_from_u64(42); // shuffling indices for random distribution
    indices.shuffle(&mut rng);


    let split_index = {total_rows as f64 * train_ratio}.floor() as usize; // .floor() is used to round the floating point number to the nearest integer
    let train_indices = &indices[..split_index]; //similar to python 0 to split_index 
    let test_indices = &indices[split_index..]; // split_index to end

    let x_train = data.get_rows(&train_indices.to_vec()); //since train_indices is in &[usize] so we convert it into Vec<usize>
    let y_train = target.get_rows(&train_indices.to_vec());
    let x_test = data.get_rows(&test_indices.to_vec());
    let y_test = target.get_rows(&test_indices.to_vec());

    let mut tree_params = decision_tree::Hyperparameters::new(data.cols()); // initializing the tree parameters...
    tree_params.min_samples_split(5).max_features(3);


    // 10 -> no of trees I want 
    let mut model = Hyperparameters::new(tree_params, 10).one_vs_rest();
    model.fit(&x_train, &y_train).unwrap();

    let predictions = model.predict(&x_test).unwrap();
    let acc = accuracy_score(&y_test, &predictions)*100.0;

    println!("Test Accuracy: {:.2}%", acc);

}
