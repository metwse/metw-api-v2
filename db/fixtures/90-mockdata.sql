-- 1 "users" mockdata
--
-- ID's from 1001 to 1020 are for users
-- ------------------------------------------------
INSERT INTO users (id, username, password, flags)
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

-- 2 "threads" for "profiles"
--
-- ID's from 2001 to 2020 are for threads
-- Owner of thread with id n is user with id (n - 1000)
-- ------------------------------------------------
INSERT INTO threads (id, user_id)
VALUES
    (2001, 1001), (2002, 1002), (2003, 1003), (2004, 1004), (2005, 1005),
    (2006, 1006), (2007, 1007), (2008, 1008), (2009, 1009), (2010, 1010),
    (2011, 1011), (2012, 1012), (2013, 1013), (2014, 1014), (2015, 1015),
    (2016, 1016), (2017, 1017), (2018, 1018), (2019, 1019), (2020, 1020);

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
