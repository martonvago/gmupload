<?php
add_action('rest_api_init', function () {
    register_rest_route('gmupload/v1', '/pieces', [
        'methods' => 'GET',
        'callback' => 'get_pieces',
        'permission_callback' => '__return_true',
    ]);
});

function get_pieces() {
    $query = new WP_Query([
        'post_type' => 'post',
        'post_status' => 'publish',
        'posts_per_page' => -1,
        'orderby' => 'title',
        'order' => 'ASC',
    ]);

    $pieces = [];

    foreach ($query->posts as $post) {
        $categories = get_the_category($post->ID);
        $category = !empty($categories) ? $categories[0]->slug : null;
        $pieces[] = [
            'title' => get_the_title($post),
            'link' => get_permalink($post),
            'content' => wp_strip_all_tags(apply_filters('the_content', $post->post_content)),
            'category' => $category,
        ];
    }

    return rest_ensure_response($pieces);
}
