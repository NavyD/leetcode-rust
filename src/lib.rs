// 允许在循环中使用索引而不是只用iter
// like this: for i in 0..nums.len() {
#![allow(clippy::needless_range_loop)]

pub mod count_of_range_sum;
pub mod count_of_smaller_numbers_after_self;
pub mod reverse_pairs;
pub mod random_pick_with_blacklist;
pub mod maximum_profit_in_job_scheduling;
pub mod sort_an_array;

// dynamic programming
pub mod climbing_stairs;
pub mod range_sum_query_immutable;
pub mod is_subsequence;
pub mod min_cost_climbing_stairs;
pub mod divisor_game;
pub mod longest_palindromic_substring;
pub mod minimum_path_sum;
pub mod decode_ways;
pub mod generate_parentheses;

pub mod fibonacci_number;
pub mod unique_paths;
pub mod unique_paths_ii;
pub mod longest_common_subsequence;
pub mod triangle;
pub mod maximum_subarray;
pub mod maximum_product_subarray;
pub mod coin_change;
pub mod coin_change_2;
pub mod house_robber;
pub mod house_robber_ii;
pub mod best_time_to_buy_and_sell_stock_ii;
pub mod best_time_to_buy_and_sell_stock;
pub mod best_time_to_buy_and_sell_stock_iii;

// tree
pub mod same_tree;
pub mod symmetric_tree;
pub mod maximum_depth_of_binary_tree;
pub mod binary_tree_level_order_traversal_ii;
pub mod convert_sorted_array_to_binary_search_tree;
pub mod balanced_binary_tree;
pub mod minimum_depth_of_binary_tree;
pub mod path_sum;
pub mod invert_binary_tree;
pub mod binary_tree_paths;
pub mod sum_of_left_leaves;
pub mod find_mode_in_binary_search_tree;
pub mod validate_binary_search_tree;
pub mod serialize_and_deserialize_binary_tree;
pub mod lowest_common_ancestor_of_a_binary_tree;
pub mod construct_binary_tree_from_preorder_and_inorder_traversal;

// tree traversal
pub mod binary_tree_inorder_traversal;
pub mod binary_tree_preorder_traversal;
pub mod binary_tree_postorder_traversal;
pub mod n_ary_tree_postorder_traversal;
pub mod binary_tree_level_order_traversal;

// recursive backtracking
pub mod combinations;
pub mod permutations;
pub mod permutations_ii;
pub mod subsets;
pub mod letter_combinations_of_a_phone_number;
pub mod n_queens;

// divide
pub mod powx_n;
pub mod majority_element;

// dfs/bfs
pub mod minimum_genetic_mutation;
pub mod find_largest_value_in_each_tree_row;
pub mod word_ladder;
pub mod word_ladder_ii;
pub mod number_of_islands;
pub mod minesweeper;

// array
pub mod move_zeroes;
pub mod container_with_most_water;
pub mod two_sum;
pub mod three_sum;
pub mod remove_duplicates_from_sorted_array;
pub mod rotate_array;
pub mod merge_sorted_array;
pub mod plus_one;
pub mod design_circular_deque;
pub mod trapping_rain_water;

// linked list
pub mod reverse_linked_list;
pub mod swap_nodes_in_pairs;
pub mod linked_list_cycle;
pub mod linked_list_cycle_ii;
pub mod reverse_nodes_in_k_group;
pub mod merge_two_sorted_lists;

// stack
pub mod valid_parentheses;
pub mod min_stack;
pub mod largest_rectangle_in_histogram;
pub mod sliding_window_maximum;

// collection mapping
pub mod valid_anagram;
pub mod group_anagrams;

pub mod helper;
pub use helper::utils as utils;

pub use leetcode_prelude as prelude;

// greedy
pub mod assign_cookies;
pub mod jump_game;
pub mod lemonade_change;
pub mod walking_robot_simulation;
pub mod jump_game_ii;

// binary search
pub mod sqrtx;
pub mod valid_perfect_square;
pub mod search_in_rotated_sorted_array;
pub mod search_a_2d_matrix;
pub mod find_minimum_in_rotated_sorted_array;
