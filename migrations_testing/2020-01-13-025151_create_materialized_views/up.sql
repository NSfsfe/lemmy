-- SPDX-FileCopyrightText: 2019-2022 2019 Felix Ableitner, <me@nutomic.com> et al.
--
-- SPDX-License-Identifier: AGPL-3.0-only

-- post
drop view post_view;
create materialized view post_view as
with all_post as
(
  select        
  p.*,
  (select u.banned from user_ u where p.creator_id = u.id) as banned,
  (select cb.id::bool from community_user_ban cb where p.creator_id = cb.user_id and p.community_id = cb.community_id) as banned_from_community,
  (select name from user_ where p.creator_id = user_.id) as creator_name,
  (select avatar from user_ where p.creator_id = user_.id) as creator_avatar,
  (select name from community where p.community_id = community.id) as community_name,
  (select removed from community c where p.community_id = c.id) as community_removed,
  (select deleted from community c where p.community_id = c.id) as community_deleted,
  (select nsfw from community c where p.community_id = c.id) as community_nsfw,
  (select count(*) from comment where comment.post_id = p.id) as number_of_comments,
  coalesce(sum(pl.score), 0) as score,
  count (case when pl.score = 1 then 1 else null end) as upvotes,
  count (case when pl.score = -1 then 1 else null end) as downvotes,
  hot_rank(coalesce(sum(pl.score) , 0), p.published) as hot_rank
  from post p
  left join post_like pl on p.id = pl.post_id
  group by p.id
)

select
ap.*,
u.id as user_id,
coalesce(pl.score, 0) as my_vote,
(select cf.id::bool from community_follower cf where u.id = cf.user_id and cf.community_id = ap.community_id) as subscribed,
(select pr.id::bool from post_read pr where u.id = pr.user_id and pr.post_id = ap.id) as read,
(select ps.id::bool from post_saved ps where u.id = ps.user_id and ps.post_id = ap.id) as saved
from user_ u
cross join all_post ap
left join post_like pl on u.id = pl.user_id and ap.id = pl.post_id

union all

select 
ap.*,
null as user_id,
null as my_vote,
null as subscribed,
null as read,
null as saved
from all_post ap
with data
;

create unique index idx_post_view_unique on post_view (id, user_id); 
create index idx_post_view_user_id on post_view (user_id);
create index idx_post_view_hot_rank_published on post_view (hot_rank desc, published desc);
create index idx_post_view_published on post_view (published desc);
create index idx_post_view_score on post_view (score desc);

-- user_view
drop view user_view;
create materialized view user_view as 
select id,
name,
avatar,
email,
fedi_name,
admin,
banned,
show_avatars,
send_notifications_to_email,
published,
(select count(*) from post p where p.creator_id = u.id) as number_of_posts,
(select coalesce(sum(score), 0) from post p, post_like pl where u.id = p.creator_id and p.id = pl.post_id) as post_score,
(select count(*) from comment c where c.creator_id = u.id) as number_of_comments,
(select coalesce(sum(score), 0) from comment c, comment_like cl where u.id = c.creator_id and c.id = cl.comment_id) as comment_score
from user_ u;

create unique index idx_user_view_unique on user_view (id); 
create index idx_user_view_comment_published on user_view (comment_score desc, published desc);
create index idx_user_view_admin on user_view (admin);
create index idx_user_view_banned on user_view (banned);

-- community
drop view community_view;
create materialized view community_view as 
with all_community as
(
  select *,
  (select name from user_ u where c.creator_id = u.id) as creator_name,
  (select avatar from user_ u where c.creator_id = u.id) as creator_avatar,
  (select name from category ct where c.category_id = ct.id) as category_name,
  (select count(*) from community_follower cf where cf.community_id = c.id) as number_of_subscribers,
  (select count(*) from post p where p.community_id = c.id) as number_of_posts,
  (select count(*) from comment co, post p where c.id = p.community_id and p.id = co.post_id) as number_of_comments,
  hot_rank((select count(*) from community_follower cf where cf.community_id = c.id), c.published) as hot_rank
  from community c
)

select
ac.*,
u.id as user_id,
(select cf.id::boolean from community_follower cf where u.id = cf.user_id and ac.id = cf.community_id) as subscribed
from user_ u
cross join all_community ac

union all

select 
ac.*,
null as user_id,
null as subscribed
from all_community ac
;

create unique index idx_community_view_unique on community_view (id, user_id); 
create index idx_community_view_user_id on community_view (user_id);
create index idx_community_view_hot_rank_subscribed on community_view (hot_rank desc, number_of_subscribers desc);


-- reply and comment view
drop view reply_view;
drop view user_mention_view;
drop view comment_view;
create materialized view comment_view as
with all_comment as
(
  select        
  c.*,
  (select community_id from post p where p.id = c.post_id),
  (select u.banned from user_ u where c.creator_id = u.id) as banned,
  (select cb.id::bool from community_user_ban cb, post p where c.creator_id = cb.user_id and p.id = c.post_id and p.community_id = cb.community_id) as banned_from_community,
  (select name from user_ where c.creator_id = user_.id) as creator_name,
  (select avatar from user_ where c.creator_id = user_.id) as creator_avatar,
  coalesce(sum(cl.score), 0) as score,
  count (case when cl.score = 1 then 1 else null end) as upvotes,
  count (case when cl.score = -1 then 1 else null end) as downvotes
  from comment c
  left join comment_like cl on c.id = cl.comment_id
  group by c.id
)

select
ac.*,
u.id as user_id,
coalesce(cl.score, 0) as my_vote,
(select cs.id::bool from comment_saved cs where u.id = cs.user_id and cs.comment_id = ac.id) as saved
from user_ u
cross join all_comment ac
left join comment_like cl on u.id = cl.user_id and ac.id = cl.comment_id

union all

select 
    ac.*,
    null as user_id, 
    null as my_vote,
    null as saved
from all_comment ac
;

create unique index idx_comment_view_unique on comment_view (id, user_id); 
create index idx_comment_view_user_id on comment_view (user_id);
create index idx_comment_view_creator_id on comment_view (creator_id);
create index idx_comment_view_post_id on comment_view (post_id);
create index idx_comment_view_score on comment_view (score desc);

create view reply_view as 
with closereply as (
    select 
    c2.id, 
    c2.creator_id as sender_id, 
    c.creator_id as recipient_id
    from comment c
    inner join comment c2 on c.id = c2.parent_id
    where c2.creator_id != c.creator_id
    -- Do union where post is null
    union
    select
    c.id,
    c.creator_id as sender_id,
    p.creator_id as recipient_id
    from comment c, post p
    where c.post_id = p.id and c.parent_id is null and c.creator_id != p.creator_id
)
select cv.*,
closereply.recipient_id
from comment_view cv, closereply
where closereply.id = cv.id
;

-- user mention
create view user_mention_view as
select 
    c.id,
    um.id as user_mention_id,
    c.creator_id,
    c.post_id,
    c.parent_id,
    c.content,
    c.removed,
    um.read,
    c.published,
    c.updated,
    c.deleted,
    c.community_id,
    c.banned,
    c.banned_from_community,
    c.creator_name,
    c.creator_avatar,
    c.score,
    c.upvotes,
    c.downvotes,
    c.user_id,
    c.my_vote,
    c.saved,
    um.recipient_id
from user_mention um, comment_view c
where um.comment_id = c.id;

-- user
create or replace function refresh_user()
returns trigger language plpgsql
as $$
begin
  refresh materialized view concurrently comment_view; -- cause of bans
  refresh materialized view concurrently post_view;
  return null;
end $$;

create trigger refresh_user
after insert or update or delete or truncate
on user_
for each statement
execute procedure refresh_user();

-- post
create or replace function refresh_post()
returns trigger language plpgsql
as $$
begin
  refresh materialized view concurrently post_view; 
  return null;
end $$;

create trigger refresh_post
after insert or update or delete or truncate
on post
for each statement
execute procedure refresh_post();

-- post_like
create or replace function refresh_post_like()
returns trigger language plpgsql
as $$
begin
  refresh materialized view concurrently post_view;
  return null;
end $$;

create trigger refresh_post_like
after insert or update or delete or truncate
on post_like
for each statement
execute procedure refresh_post_like();

-- community
create or replace function refresh_community()
returns trigger language plpgsql
as $$
begin
  refresh materialized view concurrently post_view;
  refresh materialized view concurrently community_view; 
  return null;
end $$;

create trigger refresh_community
after insert or update or delete or truncate
on community
for each statement
execute procedure refresh_community();

-- community_follower
create or replace function refresh_community_follower()
returns trigger language plpgsql
as $$
begin
  refresh materialized view concurrently community_view;
  refresh materialized view concurrently post_view;
  return null;
end $$;

create trigger refresh_community_follower
after insert or update or delete or truncate
on community_follower
for each statement
execute procedure refresh_community_follower();

-- comment
create or replace function refresh_comment()
returns trigger language plpgsql
as $$
begin
  refresh materialized view concurrently post_view;
  refresh materialized view concurrently comment_view; 
  return null;
end $$;

create trigger refresh_comment
after insert or update or delete or truncate
on comment
for each statement
execute procedure refresh_comment();

-- comment_like
create or replace function refresh_comment_like()
returns trigger language plpgsql
as $$
begin
  refresh materialized view concurrently comment_view;
  return null;
end $$;

create trigger refresh_comment_like
after insert or update or delete or truncate
on comment_like
for each statement
execute procedure refresh_comment_like();
