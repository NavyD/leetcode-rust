// 允许在循环中使用索引而不是只用iter
// like this: for i in 0..nums.len() {
#![allow(clippy::needless_range_loop)]

pub use leetcode_prelude as prelude;

pub use helper::utils;

pub mod count_of_range_sum;
pub mod count_of_smaller_numbers_after_self;
pub mod maximum_profit_in_job_scheduling;
pub mod random_pick_with_blacklist;
pub mod reverse_pairs;
pub mod sort_an_array;

// dynamic programming
pub mod climbing_stairs;
pub mod decode_ways;
pub mod divisor_game;
pub mod generate_parentheses;
pub mod is_subsequence;
pub mod longest_palindromic_substring;
pub mod min_cost_climbing_stairs;
pub mod minimum_path_sum;
pub mod range_sum_query_immutable;

pub mod best_time_to_buy_and_sell_stock;
pub mod best_time_to_buy_and_sell_stock_ii;
pub mod best_time_to_buy_and_sell_stock_iii;
pub mod best_time_to_buy_and_sell_stock_iv;
pub mod best_time_to_buy_and_sell_stock_with_cooldown;
pub mod burst_balloons;
pub mod coin_change;
pub mod coin_change_2;
pub mod edit_distance;
pub mod fibonacci_number;
pub mod frog_jump;
pub mod house_robber;
pub mod house_robber_ii;
pub mod longest_common_subsequence;
pub mod longest_valid_parentheses;
pub mod max_sum_of_rectangle_no_larger_than_k;
pub mod maximal_square;
pub mod maximum_product_subarray;
pub mod maximum_subarray;
pub mod minimum_window_substring;
pub mod palindromic_substrings;
pub mod perfect_squares;
pub mod split_array_largest_sum;
pub mod student_attendance_record_ii;
pub mod task_scheduler;
pub mod triangle;
pub mod unique_paths;
pub mod unique_paths_ii;

// tree
pub mod balanced_binary_tree;
pub mod binary_tree_level_order_traversal_ii;
pub mod binary_tree_paths;
pub mod construct_binary_tree_from_preorder_and_inorder_traversal;
pub mod convert_sorted_array_to_binary_search_tree;
pub mod find_mode_in_binary_search_tree;
pub mod invert_binary_tree;
pub mod lowest_common_ancestor_of_a_binary_tree;
pub mod maximum_depth_of_binary_tree;
pub mod minimum_depth_of_binary_tree;
pub mod path_sum;
pub mod same_tree;
pub mod serialize_and_deserialize_binary_tree;
pub mod sum_of_left_leaves;
pub mod symmetric_tree;
pub mod validate_binary_search_tree;

// tree traversal
pub mod binary_tree_inorder_traversal;
pub mod binary_tree_level_order_traversal;
pub mod binary_tree_postorder_traversal;
pub mod binary_tree_preorder_traversal;
pub mod n_ary_tree_postorder_traversal;

// recursive backtracking
pub mod combinations;
pub mod letter_combinations_of_a_phone_number;
pub mod n_queens;
pub mod permutations;
pub mod permutations_ii;
pub mod subsets;

// divide
pub mod majority_element;
pub mod powx_n;

// dfs/bfs
pub mod find_largest_value_in_each_tree_row;
pub mod minesweeper;
pub mod minimum_genetic_mutation;
pub mod number_of_islands;
pub mod word_ladder;
pub mod word_ladder_ii;

// array
pub mod container_with_most_water;
pub mod design_circular_deque;
pub mod merge_sorted_array;
pub mod move_zeroes;
pub mod plus_one;
pub mod remove_duplicates_from_sorted_array;
pub mod rotate_array;
pub mod three_sum;
pub mod trapping_rain_water;
pub mod two_sum;

// linked list
pub mod linked_list_cycle;
pub mod linked_list_cycle_ii;
pub mod merge_two_sorted_lists;
pub mod reverse_linked_list;
pub mod reverse_nodes_in_k_group;
pub mod swap_nodes_in_pairs;

// stack
pub mod largest_rectangle_in_histogram;
pub mod min_stack;
pub mod sliding_window_maximum;
pub mod valid_parentheses;

// collection mapping
pub mod group_anagrams;
pub mod valid_anagram;

pub mod helper;
// greedy
pub mod assign_cookies;
pub mod jump_game;
pub mod jump_game_ii;
pub mod lemonade_change;
pub mod walking_robot_simulation;

// binary search
pub mod find_minimum_in_rotated_sorted_array;
pub mod search_a_2d_matrix;
pub mod search_in_rotated_sorted_array;
pub mod sqrtx;
pub mod valid_perfect_square;

// trie
pub mod implement_trie_prefix_tree;
pub mod word_search;
pub mod word_search_ii;
