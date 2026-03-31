use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

fn row_passes(row: &Row, condition: &Condition, dataset: &Dataset) -> bool {
    match condition {
        Condition::Equal(column_name, target_value) => {
            let index = dataset.column_index(column_name);
            let value = row.get_value(index);
            if value == target_value {
                return true;
            } else {
                return false;
            }
        }
        Condition::Not(inner) => {
            return !row_passes(row, inner, dataset);
        }
        Condition::And(left, right) => {
            return row_passes(row, left, dataset) && row_passes(row, right, dataset);
        }
        Condition::Or(left, right) => {
            return row_passes(row, left, dataset) || row_passes(row, right, dataset);
        }
    }
}

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    let mut filtered = Dataset::new(dataset.columns().clone());
    for row in dataset.iter() {
        if row_passes(row, filter, dataset) {
            filtered.add_row(row.clone());
        }
    }
    filtered
}


pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    todo!("Implement this!");
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    let mut aggregated = HashMap::new();
    for (key, group) in dataset {
        match aggregation {
            Aggregation::Count(_) => {
                aggregated.insert(key, Value::Integer(group.len() as i32));
            }
            Aggregation::Sum(col_name) => {
                let index = group.column_index(col_name);
                let mut sum = 0;
                for row in group.iter() {
                    match row.get_value(index) {
                        Value::Integer(n) => sum += n,
                        _ => panic!("Sum on non-integer column"),
                    }
                }
                aggregated.insert(key, Value::Integer(sum));
            }
            Aggregation::Average(col_name) => {
                let index = group.column_index(col_name);
                let mut sum = 0;
                let mut count = 0;
                for row in group.iter() {
                    count += 1;
                    match row.get_value(index) {
                        Value::Integer(n) => sum += n,
                        _ => panic!("Average on non-integer column"),
                    }
                }
                aggregated.insert(key, Value::Integer(sum / count));
            }
        }
    }
    aggregated
}


pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}