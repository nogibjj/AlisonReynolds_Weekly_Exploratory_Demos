use linfa::prelude::*;
use linfa_trees::{DecisionTree, Result, SplitQuality};

pub fn decision_tree(split: String) -> Result<()> {
    let (train, test) = linfa_datasets::winequality()
        .map_targets(|x| *x > 6)
        .split_with_ratio(0.9);

        if split == "gini" {
            println!("Training model with Gini criterion ...");
            let gini_model = DecisionTree::params()
              .split_quality(SplitQuality::Gini)
              .max_depth(Some(100))
              .min_weight_split(1.0)
              .min_weight_leaf(1.0)
              .fit(&train)?;
    
            let gini_pred_y = gini_model.predict(&test);
            let cm = gini_pred_y.confusion_matrix(&test)?;
            println!("{:?}", cm);
    
            println!(
                "Test accuracy with Gini criterion: {:.2}%",
                100.0 * cm.accuracy()
            );} else if split == "entropy" {
            println!("Training model with Entropy criterion ...");
            let ent_model = DecisionTree::params()
              .split_quality(SplitQuality::Entropy)
              .max_depth(Some(100))
              .min_weight_split(1.0)
              .min_weight_leaf(1.0)
              .fit(&train)?;
    
            let ent_pred_y = ent_model.predict(&test);
            let cm = ent_pred_y.confusion_matrix(&test)?;
            println!("{:?}", cm);
    
            println!(
                "Test accuracy with Entropy criterion: {:.2}%",
                100.0 * cm.accuracy()
            )} else {
                println!("Please enter a valid criterion: gini or entropy");}
        

    Ok(())
}
