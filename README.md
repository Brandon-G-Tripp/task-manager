# Rust CLI - Task Manager with Snippet Integration

## Testing 

1. Steps to test with tarpaulin
`rust
$ cargo clean
$ cargo build
$ cargo tarpaulin --skip-clean 
`

## Roadmap for further development

### 1. Tasks Module 

v0.1 (current)

- CRUD operations
- Filtering 
- Stats
- YAML persistence
- Tests & docs

v0.2 

- Priorities
- Add priority field to tasks
- CLI option to filter by priority
- Sorting by priority

v0.3

- Tags
- Add tags field to tasks 
- Ability to add/remove tags
- Filter by tags
- Most used tags

v0.4 

- Notifications
- Schedule daily notification for due tasks
- Email, desktop notifications
- Snooze overdue tasks

v0.5 

- Calendars  
- Integrate with Google Cal, iCal
- Auto-sync calendar events as tasks
- Avoid double bookings  

v0.6 

- Collaboration
- User accounts 
- Share tasks with others
- Assign tasks to users
- Commenting

v0.7

- Analytics 
- Charts/graphs of task stats
- Productivity tracking 
- Goal setting

Some other ideas:

- Recurring tasks
- Checklists
- Attachments

### 2. Snippets Module 
### 3. Timers Module 

