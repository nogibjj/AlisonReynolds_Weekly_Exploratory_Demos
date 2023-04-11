use linfa::prelude::*;
use linfa_svm::{error::Result, Svm};

fn main() -> Result<()> {
    // everything above 6.5 is considered a good wine
    let (train, valid) = linfa_datasets::winequality()
        .map_targets(|x| *x > 6)
        .split_with_ratio(0.8);

    // fit a SVM model with a gaussian kernel and a weight of 10000 for positive samples and 1000
    let model = Svm::<_, bool>::params()
        .pos_neg_weights(10000., 1000.)
        .gaussian_kernel(75.0)
        .fit(&train)?;

    println!("{}", model);
    // A positive prediction indicates a good wine, a negative, a bad one
    fn tag_classes(x: &bool) -> String {
        if *x {
            "good wine".into()
        } else {
            "bad wine".into()
        }
    }

    // map targets for validation dataset
    let valid = valid.map_targets(tag_classes);

    // predict and map targets
    let pred = model.predict(&valid).map(tag_classes);

    // create a confusion matrix
    let cm = pred.confusion_matrix(&valid)?;

    // Print the confusion matrix
    println!("{:?}", cm);

    // Calculate the accuracy 
    println!("accuracy {}", cm.accuracy());

    Ok(())
}