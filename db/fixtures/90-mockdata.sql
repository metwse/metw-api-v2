-- 1 "users" mockdata
--
-- ID's from 1001 to 1020 are for users
-- ------------------------------------------------
INSERT INTO users (id, username, password_hash, flags)
VALUES
    (1001, 'user01', 'argon2passwd', b'00'),
    (1002, 'user02', 'argon2passwd', b'00'),
    (1003, 'user03', 'argon2passwd', b'00'),
    (1004, 'user04', 'argon2passwd', b'00'),
    (1005, 'user05', 'argon2passwd', b'00'),
    (1006, 'user06', 'argon2passwd', b'00'),
    (1007, 'user07', 'argon2passwd', b'00'),
    (1008, 'user08', 'argon2passwd', b'00'),
    (1009, 'user09', 'argon2passwd', b'00'),
    (1010, 'user10', 'argon2passwd', b'00'),
    (1011, 'user11', 'argon2passwd', b'00'),
    (1012, 'user12', 'argon2passwd', b'00'),
    (1013, 'user13', 'argon2passwd', b'00'),
    (1014, 'user14', 'argon2passwd', b'00'),
    (1015, 'user15', 'argon2passwd', b'00'),
    (1016, 'user16', 'argon2passwd', b'00'),
    (1017, 'user17', 'argon2passwd', b'00'),
    (1018, 'user18', 'argon2passwd', b'00'),
    (1019, 'user19', 'argon2passwd', b'00'),
    (1020, 'user20', 'argon2passwd', b'00');

-- 2 "threads" mockdata
--
-- Owner of thread with id x0ab is user with id 10ab
-- ------------------------------------------------
INSERT INTO threads (id, user_id)
VALUES
  -- vvvv comments_thread_id (for profiles)
    (2001, 1001), (3001, 1001),
                -- vvvv replies_thread_id (for posts)
    (2002, 1002), (3002, 1002),
    (2003, 1003), (3003, 1003),
    (2004, 1004), (3004, 1004),
    (2005, 1005), (3005, 1005),
    (2006, 1006), (3006, 1006),
    (2007, 1007), (3007, 1007),
    (2008, 1008), (3008, 1008),
    (2009, 1009), (3009, 1009),
    (2010, 1010), (3010, 1010),
    (2011, 1011), (3011, 1011),
    (2012, 1012), (3012, 1012),
    (2013, 1013), (3013, 1013),
    (2014, 1014), (3014, 1014),
    (2015, 1015), (3015, 1015),
    (2016, 1016), (3016, 1016),
    (2017, 1017), (3017, 1017),
    (2018, 1018), (3018, 1018),
    (2019, 1019), (3019, 1019),
    (2020, 1020), (3020, 1020);

-- 3 "profiles" mockdata
INSERT INTO profiles (user_id, comments_thread_id, avatar_id, banner_id, bio)
VALUES
    (1001, 2001, NULL, NULL, 'bio for user01'),
    (1002, 2002, NULL, NULL, 'bio for user02'),
    (1003, 2003, NULL, NULL, 'bio for user03'),
    (1004, 2004, NULL, NULL, 'bio for user04'),
    (1005, 2005, NULL, NULL, 'bio for user05'),
    (1006, 2006, NULL, NULL, 'bio for user06'),
    (1007, 2007, NULL, NULL, 'bio for user07'),
    (1008, 2008, NULL, NULL, 'bio for user08'),
    (1009, 2009, NULL, NULL, 'bio for user09'),
    (1010, 2010, NULL, NULL, 'bio for user10'),
    (1011, 2011, NULL, NULL, 'bio for user11'),
    (1012, 2012, NULL, NULL, 'bio for user12'),
    (1013, 2013, NULL, NULL, 'bio for user13'),
    (1014, 2014, NULL, NULL, 'bio for user14'),
    (1015, 2015, NULL, NULL, 'bio for user15'),
    (1016, 2016, NULL, NULL, 'bio for user16'),
    (1017, 2017, NULL, NULL, 'bio for user17'),
    (1018, 2018, NULL, NULL, 'bio for user18'),
    (1019, 2019, NULL, NULL, 'bio for user19'),
    (1020, 2020, NULL, NULL, 'bio for user20');

-- 4 "posts" mockdata
INSERT INTO posts
    (id, user_id, thread_id, replies_thread_id, content, is_edited, attachments)
VALUES
                 -- posts with thread_id NULL are sent to main thread
    (4001, 1001, NULL, 3001, 'post01 from user01 content', false, ARRAY[]::bigint[]),
    (4002, 1002, NULL, 3002, 'post02 from user02 content', false, ARRAY[]::bigint[]),
    (4003, 1003, NULL, 3003, 'post03 from user03 content', false, ARRAY[]::bigint[]),
    (4004, 1004, NULL, 3004, 'post04 from user04 content', false, ARRAY[]::bigint[]),
    (4005, 1005, NULL, 3005, 'post05 from user05 content', false, ARRAY[]::bigint[]),
    (4006, 1006, NULL, 3006, 'post06 from user06 content', false, ARRAY[]::bigint[]),
    (4007, 1007, NULL, 3007, 'post07 from user07 content', false, ARRAY[]::bigint[]),
    (4008, 1008, NULL, 3008, 'post08 from user08 content', false, ARRAY[]::bigint[]),
    (4009, 1009, NULL, 3009, 'post09 from user09 content', false, ARRAY[]::bigint[]),
    (4010, 1010, NULL, 3010, 'post10 from user10 content', false, ARRAY[]::bigint[]),
    (4011, 1011, NULL, 3011, 'post11 from user11 content', false, ARRAY[]::bigint[]),
    (4012, 1012, NULL, 3012, 'post12 from user12 content', false, ARRAY[]::bigint[]),
    (4013, 1013, NULL, 3013, 'post13 from user13 content', false, ARRAY[]::bigint[]),
    (4014, 1014, NULL, 3014, 'post14 from user14 content', false, ARRAY[]::bigint[]),
    (4015, 1015, NULL, 3015, 'post15 from user15 content', false, ARRAY[]::bigint[]),
    (4016, 1016, NULL, 3016, 'post16 from user16 content', false, ARRAY[]::bigint[]),
    (4017, 1017, NULL, 3017, 'post17 from user17 content', false, ARRAY[]::bigint[]),
    (4018, 1018, NULL, 3018, 'post18 from user18 content', false, ARRAY[]::bigint[]),
    (4019, 1019, NULL, 3019, 'post19 from user19 content', false, ARRAY[]::bigint[]),
    (4020, 1020, NULL, 3020, 'post20 from user20 content', false, ARRAY[]::bigint[]);
